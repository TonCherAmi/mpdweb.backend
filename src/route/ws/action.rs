use serde::Deserialize;

use crate::mpd;
use crate::route::ws::data::OneshotState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    DbUpdate { uri: Option<String> },
    QueueAdd { source: QueueSource },
    QueueReplace { source: QueueSource },
    QueueClear,
    QueueRemove { id: i64 },
    QueueNext,
    QueuePrev,
    QueueRepeat { state: bool },
    QueueConsume { state: OneshotState },
    QueueRandom { state: bool },
    QueueSingle { state: OneshotState },
    PlaybackPlay { id: Option<i64> },
    PlaybackToggle,
    PlaybackStop,
    PlaybackSeek { time: f64 },
    VolumeSet { value: u8 },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueueSource {
    File { uri: String },
    Playlist { name: String },
}

impl From<QueueSource> for mpd::QueueSource {
    fn from(source: QueueSource) -> Self {
        match source {
            QueueSource::File { uri } => mpd::QueueSource::File { uri },
            QueueSource::Playlist { name } => mpd::QueueSource::Playlist { name },
        }
    }
}
