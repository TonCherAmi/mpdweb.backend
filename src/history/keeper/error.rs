use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::mpd;
use crate::persist;

#[derive(Debug)]
pub enum Error {
    Mpd,
    Persistence,
}

impl error::Error for Error {
    // default
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Mpd => write!(f, "mpd error"),
            Error::Persistence => write!(f, "persistence error"),
        }
    }
}

impl From<mpd::Error> for Error {
    fn from(_: mpd::Error) -> Self {
        Error::Mpd
    }
}

impl From<persist::Error> for Error {
    fn from(_: persist::Error) -> Self {
        Error::Persistence
    }
}
