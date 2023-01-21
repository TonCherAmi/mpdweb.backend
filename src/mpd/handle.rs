use std::future::Future;
use std::result;
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio::time;

use crate::mpd::action::Action;
use crate::mpd::action::CoverArtKind;
use crate::mpd::action::QueueSource;
use crate::mpd::client::Client;
use crate::mpd::client::ConnectError;
use crate::mpd::data::DbCount;
use crate::mpd::data::DbItem;
use crate::mpd::data::OneshotState;
use crate::mpd::data::Playlist;
use crate::mpd::data::QueueItem;
use crate::mpd::data::Status;
use crate::mpd::data::Subsystem;
use crate::mpd::error::Error;
use crate::mpd::manager::Manager;
use crate::mpd::manager::run;
use crate::mpd::result::Result;

#[derive(Clone)]
pub struct Handle {
    action_tx: mpsc::Sender<Action>,
    idle_rx: watch::Receiver<Result<Vec<Subsystem>>>,
}

impl Handle {
    pub fn new<F, R>(connect: F) -> Self
        where
            F: Send + Sync + 'static + Fn() -> R,
            R: Send + Sync + 'static + Future<Output=result::Result<Client, ConnectError>>,
    {
        let (action_tx, action_rx) = mpsc::channel(8);
        let (idle_tx, idle_rx) = watch::channel(Ok(Vec::new()));

        let mgr = Manager::new(connect, idle_tx, action_rx);

        tokio::spawn(
            run(mgr),
        );

        Handle { action_tx, idle_rx }
    }
}

const ACTION_TIMEOUT: Duration = Duration::from_secs(10);

macro_rules! actions {
    () => {
        // nothing
    };
    ($name:ident($($pn:ident: $pt:ty),*) $(-> $rt:ty)? = $en:ident::$ev:ident; $($tail:tt)*) => {
        pub async fn $name(&self, $($pn: $pt),*) $(-> $rt)? {
            let (tx, rx) = oneshot::channel();

            let action = $en::$ev { response_tx: tx, $($pn),* };

            self.inner.action_tx.send(action).await.expect("expected to be able send action");

            tokio::select! {
                _ = time::sleep(ACTION_TIMEOUT) => {
                    return Err(Error::Unavailable("action timed out".to_owned()));
                },
                response = rx => {
                    return response.expect("expected to be able to receive action response");
                },
            }
        }

        actions!($($tail)*);
    }
}

pub struct DbHandle<'a> {
    inner: &'a Handle,
}

impl<'a> DbHandle<'a> {
    actions! {
        get(uri: String) -> Result<Vec<DbItem>> = Action::DbGet;
        count(uri: String) -> Result<DbCount> = Action::DbCount;
        search(query: String) -> Result<Vec<DbItem>> = Action::DbSearch;
        update(uri: Option<String>) -> Result<()> = Action::DbUpdate;
        cover_art(uri: String, kind: CoverArtKind) -> Result<Bytes> = Action::DbCoverArt;
    }
}

pub struct QueueHandle<'a> {
    inner: &'a Handle,
}

impl<'a> QueueHandle<'a> {
    actions! {
        get() -> Result<Vec<QueueItem>> = Action::QueueGet;
        add(source: QueueSource) -> Result<()> = Action::QueueAdd;
        replace(source: QueueSource) -> Result<()> = Action::QueueReplace;
        clear() -> Result<()> = Action::QueueClear;
        remove(id: i64) -> Result<()> = Action::QueueRemove;
        next() -> Result<()> = Action::QueueNext;
        prev() -> Result<()> = Action::QueuePrev;
        repeat(state: bool) -> Result<()> = Action::QueueRepeat;
        consume(state: OneshotState) -> Result<()> = Action::QueueConsume;
        random(state: bool) -> Result<()> = Action::QueueRandom;
        single(state: OneshotState) -> Result<()> = Action::QueueSingle;
    }
}

pub struct PlaylistHandle<'a> {
    inner: &'a Handle,
}

impl<'a> PlaylistHandle<'a> {
    actions! {
        get(name: String) -> Result<Vec<DbItem>> = Action::PlaylistsGet;
        list() -> Result<Vec<Playlist>> = Action::PlaylistsList;
        delete(name: String) -> Result<()> = Action::PlaylistsDelete;
        delete_songs(name: String, positions: Vec<usize>) -> Result<()> = Action::PlaylistsDeleteSongs;
    }
}

pub struct PlaybackHandle<'a> {
    inner: &'a Handle,
}

impl<'a> PlaybackHandle<'a> {
    actions! {
        play(id: Option<i64>) -> Result<()> = Action::PlaybackPlay;
        toggle() -> Result<()> = Action::PlaybackToggle;
        stop() -> Result<()> = Action::PlaybackStop;
        seek(time: f64) -> Result<()> = Action::PlaybackSeek;
    }
}

pub struct StatusHandle<'a> {
    inner: &'a Handle,
}

impl<'a> StatusHandle<'a> {
    actions! {
        get() -> Result<Status> = Action::StatusGet;
    }
}

pub struct VolumeHandle<'a> {
    inner: &'a Handle,
}

impl<'a> VolumeHandle<'a> {
    actions! {
        set(value: u8) -> Result<()> = Action::VolumeSet;
    }
}

impl Handle {
    pub fn db(&self) -> DbHandle {
        DbHandle { inner: self }
    }

    pub fn queue(&self) -> QueueHandle {
        QueueHandle { inner: self }
    }

    pub fn playlists(&self) -> PlaylistHandle {
        PlaylistHandle { inner: self }
    }

    pub fn playback(&self) -> PlaybackHandle {
        PlaybackHandle { inner: self }
    }

    pub fn status(&self) -> StatusHandle {
        StatusHandle { inner: self }
    }

    pub fn volume(&self) -> VolumeHandle {
        VolumeHandle { inner: self }
    }

    pub async fn changes(&mut self) -> Result<Vec<Subsystem>> {
        self.idle_rx.changed().await.expect("changes sender is dropped");

        self.idle_rx.borrow_and_update().clone()
    }
}
