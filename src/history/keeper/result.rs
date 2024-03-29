use crate::history::keeper::error;

pub type Result<T> = std::result::Result<T, error::Error>;
