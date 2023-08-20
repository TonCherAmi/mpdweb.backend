use std::error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl error::Error for Error {
    // default
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("persistence layer error: {}", self.message))
    }
}

impl From<String> for Error {
    fn from(str: String) -> Self {
        Error { message: str }
    }
}

impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error { message: err.to_string() }
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Error { message: err.to_string() }
    }
}
