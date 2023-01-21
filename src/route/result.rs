use std::result;

use crate::route::error::Error;

pub type Result<T> = result::Result<T, Error>;
