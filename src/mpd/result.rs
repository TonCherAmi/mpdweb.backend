use std::result;

use crate::mpd::error::Error;

pub type Result<T> = result::Result<T, Error>;
