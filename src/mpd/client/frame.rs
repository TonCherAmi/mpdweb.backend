use std::io::Cursor;
use std::num::ParseIntError;
use std::ops::Index;
use std::result;
use std::str;
use std::str::Utf8Error;

use bytes::Buf;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Frame {
    Ok(Bytes),
    Ver(Bytes),
    Ack(Bytes),
}

#[derive(Debug)]
pub enum Error {
    Incomplete,
    InvalidEncoding(String),
}

type Result<T> = result::Result<T, Error>;

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::InvalidEncoding(err.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::InvalidEncoding(err.to_string())
    }
}

fn get_line<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a [u8]> {
    let start = cursor.position() as usize;

    for i in start..cursor.get_ref().len() {
        if *cursor.get_ref().index(i) == b'\n' {
            cursor.set_position((i + 1) as u64);

            return Ok(&cursor.get_ref()[start..=i]);
        }
    }

    Err(Error::Incomplete)
}

const BINARY_SIZE_PREFIX: &[u8] = b"binary: ";

fn get_binary<'a>(cursor: &mut Cursor<&'a [u8]>, line: &[u8]) -> Result<Option<&'a [u8]>> {
    if !line.starts_with(BINARY_SIZE_PREFIX) {
        return Ok(None);
    }

    let start = cursor.position() as usize;

    let n = str::from_utf8(&line[BINARY_SIZE_PREFIX.len()..line.len() - 1])
        ?.parse::<usize>()?;

    // Account for the newline at the end.
    let n = n + 1;

    if cursor.remaining() < n {
        return Err(Error::Incomplete);
    }

    cursor.advance(n);

    Ok(Some(&cursor.get_ref()[start..start + n]))
}

impl Frame {
    pub fn check(cursor: &mut Cursor<&[u8]>) -> Result<()> {
        loop {
            let line = get_line(cursor)?;

            if line.starts_with(b"OK") || line.starts_with(b"ACK ") {
                return Ok(());
            }

            get_binary(cursor, line)?;
        }
    }

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Frame> {
        let mut out = BytesMut::new();

        loop {
            let line = get_line(cursor)?;

            if line.starts_with(b"OK MPD") {
                let out = &line["OK ".len()..];

                return Ok(
                    Frame::Ver(
                        Bytes::copy_from_slice(out)
                    )
                );
            }

            if line.starts_with(b"OK") {
                return Ok(
                    Frame::Ok(out.freeze())
                );
            }

            if line.starts_with(b"ACK ") {
                let out = &line["ACK ".len()..];

                return Ok(
                    Frame::Ack(
                        Bytes::copy_from_slice(out)
                    )
                );
            }

            out.put_slice(line);

            if let Some(binary) = get_binary(cursor, line)? {
                out.put_slice(binary);
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_and_parse_binary() {
        let data: &[u8] = &[0xB, 0xB];
        let buffer = [b"size: 20\nbinary: 2\n", data, b"\nOK\n"].concat();

        let mut cursor = Cursor::new(&buffer[..]);

        Frame::check(&mut cursor).unwrap();

        cursor.set_position(0);

        let frame = Frame::parse(&mut cursor).unwrap();

        let expected_contents = &buffer[..buffer.len() - 3];

        assert_eq!(frame, Frame::Ok(Bytes::copy_from_slice(expected_contents)));
    }
}
