use std::error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use bytes::Bytes;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Ack {
    pub code: i8,
    pub message: String,
    pub command: Option<String>,
    pub command_index: i16,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl error::Error for ParseError {
    // default
}

pub mod code {
    pub const _UNK: i8 = -1;

    pub const _NOT_LIST: i8 = 1;
    pub const _ARG: i8 = 2;
    pub const _PASSWORD: i8 = 3;
    pub const PERMISSION: i8 = 4;
    pub const _UNKNOWN_CMD: i8 = 5;

    pub const NO_EXIST: i8 = 50;
    pub const _PLAYLIST_MAX: i8 = 51;
    pub const _SYSTEM: i8 = 52;
    pub const _PLAYLIST_LOAD: i8 = 53;
    pub const _UPDATE_ALREADY: i8 = 54;
    pub const _PLAYER_SYNC: i8 = 55;
    pub const EXIST: i8 = 56;
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        ParseError { message }
    }
}

impl TryFrom<Bytes> for Ack {
    type Error = ParseError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let str = std::str::from_utf8(&bytes)
            .map_err(|e| e.to_string())?
            .trim();

        let left_bracket = str.find('[')
            .ok_or_else(|| format!("failed to find '[' in '{str}'"))?;

        let str = &str[left_bracket + 1..];

        let at_sign = str.find('@')
            .ok_or_else(|| format!("failed to find '@' in '{str}'"))?;

        let code = str[..at_sign].parse::<i8>()
            .map_err(|e| e.to_string())?;

        let str = &str[at_sign + 1..];

        let right_bracket = str.find(']')
            .ok_or_else(|| format!("failed to find ']' in '{str}'"))?;

        let command_index = str[..right_bracket].parse::<i16>()
            .map_err(|e| e.to_string())?;

        let str = &str[right_bracket + 1..];

        let left_bracket = str.find('{')
            .ok_or_else(|| format!("failed to find '{{' in '{str}'"))?;

        let str = &str[left_bracket + 1..];

        let right_bracket = str.find('}')
            .ok_or_else(|| format!("failed to find '}}' in '{str}'"))?;

        let command = match &str[..right_bracket] {
            "" => None,
            cmd => Some(cmd.to_owned()),
        };

        let message = str[right_bracket + 2..].to_owned();

        Ok(Ack {
            code,
            message,
            command,
            command_index,
        })
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn should_parse() {
        let bytes = Bytes::from(r#"[5@0] {} unknown command "err""#);

        let ack: Ack = bytes.try_into().unwrap();

        assert_eq!(ack, Ack {
            code: code::_UNKNOWN_CMD,
            message: r#"unknown command "err""#.to_owned(),
            command: None,
            command_index: 0,
        });
    }

    #[test]
    fn should_parse_with_command() {
        let bytes = Bytes::from(r#"[4@0] {lsinfo} you don't have permission for "lsinfo""#);

        let ack: Ack = bytes.try_into().unwrap();

        assert_eq!(ack, Ack {
            code: code::PERMISSION,
            message: r#"you don't have permission for "lsinfo""#.to_owned(),
            command: Some("lsinfo".to_owned()),
            command_index: 0,
        });
    }

    #[test]
    #[should_panic]
    fn should_not_parse() {
        let bytes = Bytes::from(r#"4@0] {lsinfo} you don't have permission for "lsinfo""#);

        let _: Ack = bytes.try_into().unwrap();
    }
}
