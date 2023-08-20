use std::env;

use crate::convert::IntoOption;

const USAGE: &str = "\
usage: mpdweb [options]

    -c, --config        path to config file\
";

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Args {
    pub config: Option<String>,
}

pub fn read() -> Result<Args, &'static str> {
    parse(env::args().collect::<Vec<_>>())
}

fn parse(args: Vec<String>) -> Result<Args, &'static str> {
    let config = match &args.iter().map(String::as_str).collect::<Vec<_>>()[1..] {
        ["-c" | "--config", config] => {
            (*config).to_owned().into_some()
        },
        [] => None,
        _ => {
            return Err(USAGE);
        }
    };

    Ok(Args { config })
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_args() {
        let expected = Args {
            config: "/home/user/.mpdweb".to_owned().into_some(),
        };

        let actual = parse(
            vec![
                "mpdweb".to_owned(),
                "-c".to_owned(),
                "/home/user/.mpdweb".to_owned()
            ]
        ).unwrap();

        assert_eq!(actual, expected);

        let actual = parse(
            vec![
                "mpdweb".to_owned(),
                "--config".to_owned(),
                "/home/user/.mpdweb".to_owned()
            ]
        ).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_empty_args() {
        let expected = Args {
            config: None,
        };

        let actual = parse(vec!["mpdweb".to_owned()]).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn should_panic_on_wrong_input_1() {
        parse(vec!["mpdweb".to_owned(), "--what".to_owned(), "value".to_owned()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn should_panic_on_wrong_input_2() {
        parse(vec!["mpdweb".to_owned(), "--config".to_owned()]).unwrap();
    }
}
