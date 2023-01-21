use serde::Deserialize;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;

use crate::mpd;
use crate::route::db::DbAudioFormat;
use crate::route::db::DbTags;
use crate::time::Duration;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaybackState {
    Playing,
    Stopped,
    Paused,
}

impl From<mpd::PlaybackState> for PlaybackState {
    fn from(state: mpd::PlaybackState) -> Self {
        use mpd::PlaybackState::*;

        match state {
            Playing => PlaybackState::Playing,
            Stopped => PlaybackState::Stopped,
            Paused => PlaybackState::Paused,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OneshotState {
    On,
    Off,
    Oneshot,
}

impl From<mpd::OneshotState> for OneshotState {
    fn from(state: mpd::OneshotState) -> Self {
        use mpd::OneshotState::*;

        match state {
            On => OneshotState::On,
            Off => OneshotState::Off,
            Oneshot => OneshotState::Oneshot,
        }
    }
}

impl From<OneshotState> for mpd::OneshotState {
    fn from(state: OneshotState) -> Self {
        use OneshotState::*;

        match state {
            On => mpd::OneshotState::On,
            Off => mpd::OneshotState::Off,
            Oneshot => mpd::OneshotState::Oneshot,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SongStatus {
    id: i64,
    position: i64,
    elapsed: Duration,
    duration: Duration,
}

impl From<mpd::SongStatus> for SongStatus {
    fn from(mpd::SongStatus { id, position, elapsed, duration }: mpd::SongStatus) -> Self {
        SongStatus {
            id,
            position,
            elapsed: elapsed.into(),
            duration: duration.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct QueueStatus {
    length: usize,
}

impl From<mpd::QueueStatus> for QueueStatus {
    fn from(mpd::QueueStatus { length }: mpd::QueueStatus) -> Self {
        QueueStatus { length }
    }
}

#[derive(Debug)]
pub enum Status {
    Disconnected,
    Connected {
        volume: i8,
        repeat: bool,
        random: bool,
        state: PlaybackState,
        single: OneshotState,
        consume: OneshotState,
        song: Option<SongStatus>,
        queue: QueueStatus,
    },
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self {
            Status::Disconnected => {
                serializer.serialize_str("disconnected")
            },
            Status::Connected {
                volume,
                repeat,
                random,
                state,
                single,
                consume,
                song,
                queue,
            } => {
                let mut s = serializer.serialize_struct("Connected", 7)?;

                s.serialize_field("volume", volume)?;
                s.serialize_field("repeat", repeat)?;
                s.serialize_field("random", random)?;
                s.serialize_field("state", state)?;
                s.serialize_field("single", single)?;
                s.serialize_field("consume", consume)?;
                s.serialize_field("song", song)?;
                s.serialize_field("queue", queue)?;

                s.end()
            }
        }
    }
}

impl From<mpd::Status> for Status {
    fn from(mpd::Status {
        volume,
        repeat,
        random,
        state,
        single,
        consume,
        song,
        queue
    }: mpd::Status) -> Self {
        Status::Connected {
            volume,
            repeat,
            random,
            state: state.into(),
            single: single.into(),
            consume: consume.into(),
            song: song.map(Into::into),
            queue: queue.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct QueueItem {
    id: i64,
    position: i64,
    uri: String,
    duration: Duration,
    tags: DbTags,
    format: Option<DbAudioFormat>,
}

impl From<mpd::QueueItem> for QueueItem {
    fn from(mpd::QueueItem {
        id,
        position,
        uri,
        duration,
        tags,
        format,
    }: mpd::QueueItem) -> Self {
        QueueItem {
            id,
            position,
            uri,
            tags: tags.into(),
            duration: duration.into(),
            format: format.map(Into::into),
        }
    }
}
