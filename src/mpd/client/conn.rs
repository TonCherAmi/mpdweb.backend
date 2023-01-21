use std::error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Cursor;
use std::result;

use bytes::Buf;
use bytes::BytesMut;
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::mpd::client::frame;
use crate::mpd::client::frame::Frame;

pub struct Connection {
    buffer: BytesMut,
    stream: TcpStream,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Closed,
    Incomplete(String),
    InvalidEncoding(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "io error: {err}"),
            Error::Closed => f.write_str("connection closed"),
            Error::Incomplete(msg) => write!(f, "incomplete frame {msg}"),
            Error::InvalidEncoding(msg) => write!(f, "invalid encoding {msg}"),
        }
    }
}

impl error::Error for Error {
    // default
}

type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl Connection {
    const BUF_CAPACITY: usize = 2048;

    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(Connection::BUF_CAPACITY),
        }
    }
}

impl Connection {
    pub async fn read_frame(&mut self) -> Result<Frame> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(frame);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Err(Error::Closed)
                } else {
                    Err(Error::Incomplete("connection closed with incomplete frame".to_owned()))
                };
            }
        }
    }

    pub async fn write_command(&mut self, cmd: &str) -> Result<()> {
        let command = [cmd.as_bytes(), b"\n"].concat();

        self.stream.write_all(&command).await?;

        Ok(())
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut cursor = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut cursor) {
            Ok(_) => {
                let len = cursor.position() as usize;

                cursor.set_position(0);

                let frame = Frame::parse(&mut cursor)
                    .expect("frame failed to parse after being checked");

                self.buffer.advance(len);

                Ok(Some(frame))
            },
            Err(frame::Error::Incomplete) => Ok(None),
            Err(frame::Error::InvalidEncoding(message)) => Err(Error::InvalidEncoding(message)),
        }
    }
}
