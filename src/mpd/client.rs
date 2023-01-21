use std::collections::HashMap;
use std::error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseFloatError;
use std::result;

use serde::de::DeserializeOwned;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use tokio::io;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

pub use crate::mpd::client::ack::Ack;
use crate::mpd::client::cmd::Command;
use crate::mpd::client::conn::Connection;
pub use crate::mpd::client::conn::Error as ConnectionError;
use crate::mpd::client::frame::Frame;

mod cmd;
mod conn;
mod de;
mod frame;
pub mod ack;

pub struct Client {
    connection: Connection,
}

pub struct CommandListClient {
    command_list: Vec<Command>,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Ack(Ack),
    Parse(String),
    Connection(conn::Error),
    Deserialization(de::Error),
}

#[derive(Debug)]
pub struct ConnectError {
    message: String,
}

impl Display for ConnectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl error::Error for ConnectError {
    // default
}

impl From<Error> for ConnectError {
    fn from(err: Error) -> Self {
        let message = match err {
            Error::Ack(ack) => ack.message,
            Error::Parse(msg) => msg,
            Error::Connection(err) => err.to_string(),
            Error::Deserialization(err) => err.to_string(),
        };

        ConnectError { message }
    }
}

impl From<io::Error> for ConnectError {
    fn from(err: io::Error) -> Self {
        ConnectError { message: err.to_string() }
    }
}

impl From<conn::Error> for ConnectError {
    fn from(err: conn::Error) -> Self {
        ConnectError { message: err.to_string() }
    }
}

impl From<ack::ParseError> for Error {
    fn from(err: ack::ParseError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<conn::Error> for Error {
    fn from(err: conn::Error) -> Self {
        Error::Connection(err)
    }
}

impl From<de::Error> for Error {
    fn from(err: de::Error) -> Self {
        Error::Deserialization(err)
    }
}

impl CommandListClient {
    fn new() -> Self {
        CommandListClient {
            command_list: Vec::new(),
        }
    }
}

impl CommandListClient {
    pub fn add(self, uri: String) -> Self {
        self.push(Command::Add { uri })
    }

    pub fn load(self, name: String) -> Self {
        self.push(Command::Load { name })
    }

    pub fn clear(self) -> Self {
        self.push(Command::Clear)
    }

    pub fn playid(self, song_id: Option<i64>) -> Self {
        self.push(Command::Playid { song_id })
    }

    pub fn playlistdelete(self, name: String, songpos: usize) -> Self {
        self.push(Command::Playlistdelete { name, songpos })
    }
}

impl CommandListClient {
    fn push(mut self, command: Command) -> Self {
        self.command_list.push(command);

        self
    }
}

impl CommandListClient {
    fn into_command(self) -> Command {
        Command::CommandList(self.command_list)
    }
}

#[derive(Deserialize)]
pub struct Status {
    pub volume: i8,
    pub repeat: i8,
    pub random: i8,
    pub single: String,
    pub consume: String,
    pub state: String,
    pub elapsed: Option<f64>,
    pub duration: Option<f64>,
    pub song: Option<i64>,
    pub songid: Option<i64>,
    pub playlistlength: usize,
}

#[derive(Deserialize)]
pub struct Change {
    pub changed: String,
}

#[derive(Deserialize)]
pub struct DbCount {
    pub songs: i64,
    pub playtime: u64,
}

pub enum DbItem {
    File {
        file: String,
        duration: Option<f64>,
        title: Vec<String>,
        artist: Vec<String>,
        album: Vec<String>,
        format: Option<String>,
    },
    Directory {
        directory: String,
    },
    Playlist {
        playlist: String,
    },
}

impl<'de> Deserialize<'de> for DbItem {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct DbItemVisitor;

        impl<'de> Visitor<'de> for DbItemVisitor {
            type Value = DbItem;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("enum DbItem")
            }

            fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
                where A: MapAccess<'de>
            {
                let mut data: HashMap<String, Vec<String>> = HashMap::new();

                while let Some((key, value)) = map.next_entry()? {
                    data.entry(key)
                        .or_default()
                        .push(value);
                }

                let first = |mut xs: Vec<String>| xs.swap_remove(0);

                let value = if data.contains_key("file") {
                    DbItem::File {
                        file: data.remove("file")
                            .map(first)
                            .ok_or_else(|| serde::de::Error::missing_field("file"))?,
                        duration: data.remove("duration")
                            .map(first)
                            .map(|it| {
                                it.parse::<f64>()
                                    .map_err(|e: ParseFloatError| serde::de::Error::custom(e.to_string()))
                            })
                            .transpose()?,
                        title: data.remove("Title").unwrap_or_default(),
                        artist: data.remove("Artist").unwrap_or_default(),
                        album: data.remove("Album").unwrap_or_default(),
                        format: data.remove("Format").map(first),
                    }
                } else if data.contains_key("directory") {
                    DbItem::Directory {
                        directory: data.remove("directory")
                            .map(first)
                            .ok_or_else(|| serde::de::Error::missing_field("directory"))?,
                    }
                } else {
                    DbItem::Playlist {
                        playlist: data.remove("playlist")
                            .map(first)
                            .ok_or_else(|| serde::de::Error::missing_field("playlist"))?,
                    }
                };

                Ok(value)
            }
        }

        deserializer.deserialize_map(DbItemVisitor)
    }
}

#[derive(Deserialize)]
pub struct PlaylistItem {
    #[serde(rename(deserialize = "Id"))]
    pub id: i64,
    #[serde(rename(deserialize = "Pos"))]
    pub pos: i64,
    pub file: String,
    pub duration: f64,
    #[serde(default, rename(deserialize = "Title"))]
    pub title: Vec<String>,
    #[serde(default, rename(deserialize = "Artist"))]
    pub artist: Vec<String>,
    #[serde(default, rename(deserialize = "Album"))]
    pub album: Vec<String>,
    #[serde(rename(deserialize = "Format"))]
    pub format: Option<String>,
}

#[derive(Deserialize)]
pub struct Playlist {
    pub playlist: String,
    #[serde(rename(deserialize = "Last-Modified"))]
    pub last_modified: String,
}

#[derive(Deserialize)]
pub struct BinaryInfo {
    pub size: usize,
    pub binary: usize,
}

#[derive(Deserialize)]
pub struct Binary(
    pub BinaryInfo,
    #[serde(with = "serde_bytes")]
    pub Vec<u8>,
);

pub async fn connect<T: ToSocketAddrs>(addr: T) -> result::Result<Client, ConnectError> {
    let stream = TcpStream::connect(addr).await?;

    let mut connection = Connection::new(stream);

    let Frame::Ver(_) = connection.read_frame().await? else {
        return Err(ConnectError { message: "unexpected frame".to_owned() });
    };

    Ok(Client { connection })
}

macro_rules! commands {
    () => {
        // nothing
    };
    ($name:ident($($pn:ident: $pt:ty),*) -> $rt:ty = $en1:ident::$en2:ident; $($tail:tt)*) => {
        pub async fn $name(&mut self, $($pn: $pt),*) -> $rt {
            let cmd = $en1::$en2 { $($pn),*  };

            self.send(&cmd.into_prepared()).await?;

            self.recv().await
        }

        commands!($($tail)*);
    };
}

impl Client {
    commands! {
        add(uri: String) -> Result<()> = Command::Add;
        load(name: String) -> Result<()> = Command::Load;
        noidle() -> Result<Vec<Change>> = Command::Noidle;
        clear() -> Result<()> = Command::Clear;
        deleteid(songid: i64) -> Result<()> = Command::Deleteid;
        playid(song_id: Option<i64>) -> Result<()> = Command::Playid;
        pause() -> Result<()> = Command::Pause;
        stop() -> Result<()> = Command::Stop;
        next() -> Result<()> = Command::Next;
        previous() -> Result<()> = Command::Previous;
        count(filter: String) -> Result<DbCount> = Command::Count;
        lsinfo(uri: String) -> Result<Vec<DbItem>> = Command::Lsinfo;
        search(filter: String) -> Result<Vec<DbItem>> = Command::Search;
        playlistinfo() -> Result<Vec<PlaylistItem>> = Command::Playlistinfo;
        listplaylists() -> Result<Vec<Playlist>> = Command::Listplaylists;
        listplaylistinfo(name: String) -> Result<Vec<DbItem>> = Command::Listplaylistinfo;
        rm(name: String) -> Result<()> = Command::Rm;
        status() -> Result<Status> = Command::Status;
        password(str: String) -> Result<()> = Command::Password;
        update(uri: Option<String>) -> Result<()> = Command::Update;
        seekcur(time: String) -> Result<()> = Command::Seekcur;
        setvol(vol: u8) -> Result<()> = Command::Setvol;
        albumart(uri: String, offset: usize) -> Result<Binary> = Command::Albumart;
        readpicture(uri: String, offset: usize) -> Result<Option<Binary>> = Command::Readpicture;
        repeat(state: String) -> Result<()> = Command::Repeat;
        consume(state: String) -> Result<()> = Command::Consume;
        random(state: String) -> Result<()> = Command::Random;
        single(state: String) -> Result<()> = Command::Single;
    }

    pub async fn idle<F: FnMut()>(&mut self, subsystems: Vec<String>, mut on_idle: F) -> Result<Vec<Change>> {
        let cmd = Command::Idle { subsystems };

        self.send(&cmd.into_prepared()).await?;

        on_idle();

        self.recv().await
    }

    pub async fn command_list<F>(&mut self, builder: F) -> Result<()>
        where F: FnOnce(CommandListClient) -> CommandListClient
    {
        let cmd = builder(CommandListClient::new())
            .into_command()
            .into_prepared();

        self.send(&cmd).await?;

        self.recv().await
    }
}

impl Client {
    async fn send(&mut self, cmd: &str) -> Result<()> {
        tracing::trace!("writing command: {cmd}");

        self.connection.write_command(cmd).await?;

        tracing::trace!("wrote command: {cmd}");

        Ok(())
    }

    async fn recv<T: DeserializeOwned>(&mut self) -> Result<T> {
        tracing::trace!("reading response");

        let response = self.connection.read_frame().await?;

        tracing::trace!("read response: {:?}", response);

        match response {
            Frame::Ok(bytes) => Ok(de::from_bytes(&bytes)?),
            Frame::Ver(_) => panic!("unexpected version frame"),
            Frame::Ack(bytes) => Err(Error::Ack(bytes.try_into()?)),
        }
    }
}
