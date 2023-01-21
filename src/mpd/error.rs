use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::mpd::client;
use crate::mpd::client::ConnectionError;

#[derive(Clone, Debug)]
pub enum Error {
    Internal(String),
    Unavailable(String),
    Disconnected(String),
    Forbidden(String),
    NotFound(String),
    AlreadyExists(String),
}

impl error::Error for Error {
    // default
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Internal(msg) => write!(f, "internal error: {msg}"),
            Unavailable(msg) => write!(f, "unavailable: {msg}"),
            Forbidden(msg) => write!(f, "unauthorized: {msg}"),
            NotFound(msg) => write!(f, "not found: {msg}"),
            AlreadyExists(msg) => write!(f, "already exists: {msg}"),
            Disconnected(msg) => write!(f, "disconnected: {msg}"),
        }
    }
}

impl From<client::Ack> for Error {
    fn from(ack: client::Ack) -> Self {
        match ack.code {
            client::ack::code::PERMISSION => Error::Forbidden(ack.message),
            client::ack::code::NO_EXIST => Error::NotFound(ack.message),
            client::ack::code::EXIST => Error::AlreadyExists(ack.message),
            _ => Error::Internal(ack.message),
        }
    }
}

impl From<client::Error> for Error {
    fn from(err: client::Error) -> Self {
        use client::Error::*;

        match err {
            Ack(ack) => ack.into(),
            Connection(ConnectionError::Closed) => Error::Disconnected("connection closed".to_owned()),
            Connection(err) => Error::Internal(err.to_string()),
            Parse(msg) => Error::Internal(msg),
            Deserialization(err) => Error::Internal(err.to_string()),
        }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Internal(s)
    }
}
