use std::str::FromStr;
use std::time::Duration;

use crate::mpd::client;
use crate::mpd::Error;

#[derive(Debug)]
pub enum PlaybackState {
    Playing,
    Stopped,
    Paused,
}

impl TryFrom<String> for PlaybackState {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = match value.as_str() {
            "play" => PlaybackState::Playing,
            "stop" => PlaybackState::Stopped,
            "pause" => PlaybackState::Paused,
            _ => return Err(format!("unknown state '{value}'")),
        };

        Ok(value)
    }
}

#[derive(Debug)]
pub enum OneshotState {
    On,
    Off,
    Oneshot,
}

impl TryFrom<String> for OneshotState {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = match value.as_str() {
            "1" => OneshotState::On,
            "0" => OneshotState::Off,
            "oneshot" => OneshotState::Oneshot,
            _ => return Err(format!("unknown oneshot state '{value}'")),
        };

        Ok(value)
    }
}

pub fn to_state_string(state: bool) -> String {
    let result = if state { "1" } else { "0" };

    result.to_owned()
}

impl OneshotState {
    pub fn to_state_string(&self) -> String {
        let result = match self {
            OneshotState::On => "1",
            OneshotState::Off => "0",
            OneshotState::Oneshot => "oneshot",
        };

        result.to_owned()
    }
}

#[derive(Debug)]
pub struct SongStatus {
    pub id: i64,
    pub position: i64,
    pub elapsed: Duration,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct QueueStatus {
    pub length: usize,
}

#[derive(Debug)]
pub struct Status {
    pub volume: i8,
    pub repeat: bool,
    pub random: bool,
    pub state: PlaybackState,
    pub single: OneshotState,
    pub consume: OneshotState,
    pub song: Option<SongStatus>,
    pub queue: QueueStatus,
}

fn to_bool(value: i8) -> Result<bool, String> {
    let value = match value {
        0 => false,
        1 => true,
        _ => return Err(format!("unknown value '{value}'"))
    };

    Ok(value)
}

impl TryFrom<client::Status> for Status {
    type Error = String;

    fn try_from(status: client::Status) -> Result<Self, Self::Error> {
        let status = Status {
            volume: status.volume,
            repeat: to_bool(status.repeat)?,
            random: to_bool(status.random)?,
            single: status.single.try_into()?,
            consume: status.consume.try_into()?,
            state: status.state.try_into()?,
            song: match (status.songid, status.song, status.elapsed, status.duration) {
                (Some(id), Some(position), Some(elapsed), Some(duration)) => {
                    Some(SongStatus {
                        id,
                        position,
                        elapsed: Duration::from_secs_f64(elapsed),
                        duration: Duration::from_secs_f64(duration),
                    })
                }
                _ => None,
            },
            queue: QueueStatus {
                length: status.playlistlength,
            },
        };

        Ok(status)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Subsystem {
    Database,
    Playlist,
    Queue,
    Player,
    Volume,
    Options,
}

impl Subsystem {
    const DATABASE_VALUE: &'static str = "database";
    const PLAYLIST_VALUE: &'static str = "stored_playlist";
    const QUEUE_VALUE: &'static str = "playlist";
    const VOLUME_VALUE: &'static str = "mixer";
    const PLAYER_VALUE: &'static str = "player";
    const OPTIONS_VALUE: &'static str = "options";
}

impl TryFrom<&str> for Subsystem {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Subsystem::*;

        let variant = match s {
            Subsystem::DATABASE_VALUE => Database,
            Subsystem::PLAYLIST_VALUE => Playlist,
            Subsystem::QUEUE_VALUE => Queue,
            Subsystem::PLAYER_VALUE => Player,
            Subsystem::VOLUME_VALUE => Volume,
            Subsystem::OPTIONS_VALUE => Options,
            _ => return Err(format!("unknown subsystem '{s}'"))
        };

        Ok(variant)
    }
}

impl From<Subsystem> for String {
    fn from(subsystem: Subsystem) -> Self {
        use Subsystem::*;

        let value = match subsystem {
            Database => Subsystem::DATABASE_VALUE,
            Playlist => Subsystem::PLAYLIST_VALUE,
            Queue => Subsystem::QUEUE_VALUE,
            Player => Subsystem::PLAYER_VALUE,
            Volume => Subsystem::VOLUME_VALUE,
            Options => Subsystem::OPTIONS_VALUE,
        };

        value.to_owned()
    }
}

pub fn to_subsystems(changes: Vec<client::Change>) -> Result<Vec<Subsystem>, String> {
    changes.into_iter()
        .map(|it| it.changed.as_str().try_into())
        .collect()
}

#[derive(Debug)]
pub struct DbCount {
    pub nsongs: i64,
    pub playtime: Duration,
}

impl From<client::DbCount> for DbCount {
    fn from(count: client::DbCount) -> Self {
        DbCount {
            nsongs: count.songs,
            playtime: Duration::from_secs(count.playtime),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, Eq, PartialEq))]
pub struct DbAudioFormat {
    pub bit_depth: i64,
    pub sampling_rate: i64,
    pub number_of_channels: i64,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, Eq, PartialEq))]
pub struct DbTags {
    pub titles: Vec<String>,
    pub artists: Vec<String>,
    pub albums: Vec<String>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, Eq, PartialEq))]
pub enum DbItem {
    File {
        uri: String,
        duration: Option<Duration>,
        tags: DbTags,
        format: Option<DbAudioFormat>,
    },
    Directory {
        uri: String,
    },
    Playlist {
        uri: String,
    },
}

impl FromStr for DbAudioFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<&str>>();

        if parts.len() != 3 {
            return Err(format!("invalid number of parts in format string {s}"))
        }

        Ok(DbAudioFormat {
            bit_depth: parts[1].parse().unwrap_or(-1),
            sampling_rate: parts[0].parse().unwrap_or(-1),
            number_of_channels: parts[2].parse().unwrap_or(-1),
        })
    }
}

impl TryFrom<client::DbItem> for DbItem {
    type Error = Error;

    fn try_from(value: client::DbItem) -> Result<Self, Self::Error> {
        use client::DbItem::*;

        let result = match value {
            File {
                file,
                duration,
                title,
                artist,
                album,
                format,
            } => {
                DbItem::File {
                    uri: file,
                    duration: duration.map(Duration::from_secs_f64),
                    tags: DbTags {
                        titles: title,
                        artists: artist,
                        albums: album,
                    },
                    format: format.map(|s| s.parse()).transpose()?,
                }
            }
            Directory { directory } => DbItem::Directory { uri: directory },
            Playlist { playlist } => DbItem::Playlist { uri: playlist },
        };

        Ok(result)
    }
}

impl DbItem {
    pub fn uri(&self) -> &str {
        match self {
            | DbItem::File { uri, .. }
            | DbItem::Directory { uri }
            | DbItem::Playlist { uri } => uri,
        }
    }
}

#[derive(Debug)]
pub struct QueueItem {
    pub id: i64,
    pub position: i64,
    pub uri: String,
    pub duration: Duration,
    pub tags: DbTags,
    pub format: Option<DbAudioFormat>,
}

impl TryFrom<client::PlaylistItem> for QueueItem {
    type Error = Error;

    fn try_from(
        client::PlaylistItem {
            id,
            pos,
            file,
            duration,
            title,
            artist,
            album,
            format,
        }: client::PlaylistItem
    ) -> Result<Self, Self::Error> {
        let result = QueueItem {
            id,
            position: pos,
            uri: file,
            duration: Duration::from_secs_f64(duration),
            tags: DbTags {
                titles: title,
                artists: artist,
                albums: album,
            },
            format: format.map(|s| s.parse()).transpose()?,
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub updated_at: String,
}

impl From<client::Playlist> for Playlist {
    fn from(client::Playlist { playlist, last_modified }: client::Playlist) -> Self {
        Playlist {
            name: playlist,
            updated_at: last_modified,
        }
    }
}
