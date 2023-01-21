use proc_macro::TokenStream;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use quote::quote;

const FRONTEND_OUT_DIR: Option<&str> = option_env!("MPDWEB_FRONTEND_OUT_DIR");

fn read_dir(path: &Path) -> io::Result<Vec<PathBuf>> {
    if !path.is_dir() {
        panic!("expected path to directory");
    }

    let mut result = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if entry.path().is_dir() {
            result.append(&mut read_dir(&entry.path())?);
        } else {
            result.push(entry.path());
        }
    }

    Ok(result)
}

#[proc_macro]
pub fn assets(_: TokenStream) -> TokenStream {
    let Some(directory) = FRONTEND_OUT_DIR else {
        return quote!(None).into();
    };

    let result = read_dir(Path::new(directory))
        .expect("expected read_dir to succeed");

    let mut tokens = Vec::new();

    tokens.push(quote!(let mut result: std::collections::HashMap<_, &[u8]> = std::collections::HashMap::new()));

    for path in result {
        let relative_path = path
            .to_str()
            .expect("expected UTF-8 encodable path")
            .trim_start_matches(directory)
            .trim_start_matches('/');

        let absolute_path = path.canonicalize()
            .expect("expected to be able to canonicalize path");

        let absolute_path = absolute_path
            .to_str()
            .expect("expected UTF-8 encodable path");

        tokens.push(quote!(result.insert(#relative_path, include_bytes!(#absolute_path))));
    }

    tokens.push(quote!(Some(result)));

    quote!({ #(#tokens);* }).into()
}
