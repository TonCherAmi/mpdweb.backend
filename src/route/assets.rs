use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

use axum::Extension;
use axum::http::header;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::http::header::HeaderName;

use crate::route::error::Error;
use crate::route::result::Result;

fn ext(path: &str) -> Option<&str> {
    Path::new(path).extension().and_then(OsStr::to_str)
}

fn mime_type(ext: &str) -> &'static str {
    match ext {
        "ico" => "image/vnd.microsoft.icon",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "txt" => "text/plain",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        _ => "application/octet-stream",
    }
}

fn content_type(mime_type: &str) -> (HeaderName, &str) {
    (header::CONTENT_TYPE, mime_type)
}

pub async fn assets(
    uri: Uri,
    Extension(files): Extension<HashMap<&str, &'static [u8]>>,
) -> Result<([(HeaderName, &'static str); 1], &'static [u8])> {
    let not_found = || {
        Error::new(StatusCode::NOT_FOUND, format!("asset at uri '{uri}' was not found"))
    };

    let index = || {
        files.get("index.html")
            .map(|&file| ([content_type(mime_type("html"))], file))
    };

    let result = match uri.path() {
        "/" | "/index.html" => {
            index().ok_or_else(not_found)?
        },
        path => {
            // Trim leading slash.
            let path = &path[1..];

            files.get(path)
                .map(|&file| ([content_type(mime_type(ext(path).unwrap_or("")))], file))
                .or_else(index)
                .ok_or_else(not_found)?
        },
    };

    Ok(result)
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_no_ext() {
        let path = "file.d/path";

        let ext = ext(path);

        assert_eq!(ext, None);
    }

    #[test]
    fn should_be_correct_ext() {
        let path = "file/path.ttf";

        let ext = ext(path);

        assert_eq!(ext, Some("ttf"));
    }
}
