use std::result;

use crate::persist::error::Error;

pub type Result<T> = result::Result<T, Error>;
