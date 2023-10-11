use bytes::Bytes;
use tokio::sync::oneshot;

use crate::mpd::data::QueueItem;
use crate::mpd::data::Playlist;
use crate::mpd::data::OneshotState;
use crate::mpd::data::DbItem;
use crate::mpd::data::DbCount;
use crate::mpd::data::Status;
use crate::mpd::result::Result;

type ResponseSender<T> = oneshot::Sender<Result<T>>;

#[derive(Debug)]
pub enum CoverArtKind {
    File,
    Embedded,
}

#[derive(Debug)]
pub enum QueueSource {
    File { uri: String },
    Playlist { name: String },
}

#[derive(Debug)]
pub enum Action {
    // Database actions.
    DbGet {
        uri: String,
        response_tx: ResponseSender<Vec<DbItem>>,
    },
    DbCount {
        uri: String,
        response_tx: ResponseSender<DbCount>,
    },
    DbSearch {
        query: String,
        response_tx: ResponseSender<Vec<DbItem>>,
    },
    DbRecents {
      response_tx: ResponseSender<Vec<DbItem>>,
    },
    DbUpdate {
        uri: Option<String>,
        response_tx: ResponseSender<()>,
    },
    DbCoverArt {
        uri: String,
        kind: CoverArtKind,
        response_tx: ResponseSender<Bytes>,
    },
    // Queue actions.
    QueueGet {
        response_tx: ResponseSender<Vec<QueueItem>>,
    },
    QueueAdd {
        sources: Vec<QueueSource>,
        response_tx: ResponseSender<()>,
    },
    QueueReplace {
        sources: Vec<QueueSource>,
        response_tx: ResponseSender<()>,
    },
    QueueClear {
        response_tx: ResponseSender<()>,
    },
    QueueRemove {
        id: i64,
        response_tx: ResponseSender<()>,
    },
    QueueNext {
        response_tx: ResponseSender<()>,
    },
    QueuePrev {
        response_tx: ResponseSender<()>,
    },
    QueueRepeat {
        state: bool,
        response_tx: ResponseSender<()>,
    },
    QueueConsume {
        state: OneshotState,
        response_tx: ResponseSender<()>,
    },
    QueueRandom {
        state: bool,
        response_tx: ResponseSender<()>,
    },
    QueueSingle {
        state: OneshotState,
        response_tx: ResponseSender<()>,
    },
    // Playlist actions.
    PlaylistsGet {
        name: String,
        response_tx: ResponseSender<Vec<DbItem>>,
    },
    PlaylistsList {
        response_tx: ResponseSender<Vec<Playlist>>,
    },
    PlaylistsDelete {
        name: String,
        response_tx: ResponseSender<()>,
    },
    PlaylistsDeleteSongs {
        name: String,
        positions: Vec<usize>,
        response_tx: ResponseSender<()>,
    },
    // Playback actions.
    PlaybackPlay {
        id: Option<i64>,
        response_tx: ResponseSender<()>,
    },
    PlaybackToggle {
        response_tx: ResponseSender<()>,
    },
    PlaybackStop {
        response_tx: ResponseSender<()>,
    },
    PlaybackSeek {
        time: f64,
        response_tx: ResponseSender<()>,
    },
    // Status actions.
    StatusGet {
        response_tx: ResponseSender<Status>,
    },
    // Volume actions.
    VolumeSet {
        value: u8,
        response_tx: ResponseSender<()>,
    },
}
