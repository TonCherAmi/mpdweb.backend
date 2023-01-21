use std::convert;
use std::future::Future;
use std::iter::Iterator;
use std::ops::ControlFlow;
use std::result;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::watch;
use tokio::time;

use crate::mpd::action::Action;
use crate::mpd::client;
use crate::mpd::client::Client;
use crate::mpd::client::ConnectError;
use crate::mpd::data::Subsystem;
use crate::mpd::error::Error;
use crate::mpd::result::Result;

pub struct Manager<T: Fn() -> F, F: Future<Output=result::Result<Client, ConnectError>>> {
    connect: T,
    idle_tx: watch::Sender<Result<Vec<Subsystem>>>,
    action_rx: mpsc::Receiver<Action>,
}

impl<T: Fn() -> F, F: Future<Output=result::Result<Client, ConnectError>>> Manager<T, F> {
    pub fn new(connect: T, idle_tx: watch::Sender<Result<Vec<Subsystem>>>, action_rx: mpsc::Receiver<Action>) -> Self {
        Manager { connect, idle_tx, action_rx }
    }
}

pub async fn run<T, F>(mut manager: Manager<T, F>)
    where T: Fn() -> F,
          F: Future<Output=result::Result<Client, ConnectError>>,
{
    let mut recv_loop = recv::RecvLoop::new(&mut manager);

    recv_loop.run().await;
}

mod recv {
    use crate::mpd::data::to_subsystems;
    use crate::mpd::service::Service;

    use super::*;

    const IDLE_SUBSYSTEMS: &[Subsystem] = &[
        Subsystem::Database,
        Subsystem::Playlist,
        Subsystem::Queue,
        Subsystem::Volume,
        Subsystem::Player,
        Subsystem::Options,
    ];

    const RECONNECT_TIMEOUT: Duration = Duration::from_secs(5);

    pub struct RecvLoop<'a, T: Fn() -> F, F: Future<Output=result::Result<Client, ConnectError>>> {
        manager: &'a mut Manager<T, F>,
    }

    impl<'a, T: Fn() -> F, F: Future<Output=result::Result<Client, ConnectError>>> RecvLoop<'a, T, F> {
        pub fn new(manager: &'a mut Manager<T, F>) -> RecvLoop<'a, T, F> {
            RecvLoop { manager }
        }
    }

    fn as_control_flow<T>(result: &Result<T>) -> ControlFlow<()> {
        if let Err(Error::Disconnected(_)) = result {
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    }

    fn idle_input() -> Vec<String> {
        IDLE_SUBSYSTEMS.iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    impl<'a, T: Fn() -> F, F: Future<Output=result::Result<Client, ConnectError>>> RecvLoop<'a, T, F> {
        fn handle_changes(&self, changes: client::Result<Vec<client::Change>>) -> ControlFlow<()> {
            let result = changes
                .map_err(Into::into)
                .map(to_subsystems)
                .map(|it| it.map_err(Error::Internal))
                .and_then(convert::identity);

            // We usually expect to get no changes from
            // noidle so just continue if that's the case.
            if matches!(&result, Ok(xs) if xs.is_empty()) {
                return ControlFlow::Continue(());
            }

            if let Err(e) = &result {
                tracing::error!("encountered error while handling changes: {e}");
            }

            let control_flow = as_control_flow(&result);

            self.manager.idle_tx.send(result)
                .expect("expected to be able to propagate changes");

            control_flow
        }

        async fn handle_action(&self, client: &mut Client, action: Action) -> ControlFlow<()> {
            macro_rules! send {
                ($response_tx:ident << $result:expr) => {
                    let result = $result;

                    let control_flow = as_control_flow(&result);

                    let _ = $response_tx.send(result);

                    control_flow
                };
            }

            let mut service = Service::new(client);

            match action {
                // Database actions.
                Action::DbGet { uri, response_tx } => {
                    send! {
                        response_tx << service.db().get(uri).await
                    }
                },
                Action::DbCount { uri, response_tx } => {
                    send! {
                        response_tx << service.db().count(uri).await
                    }
                },
                Action::DbSearch { query, response_tx } => {
                    send! {
                        response_tx << service.db().search(query).await
                    }
                },
                Action::DbUpdate { uri, response_tx } => {
                    send! {
                        response_tx << service.db().update(uri).await
                    }
                },
                Action::DbCoverArt { uri, kind, response_tx } => {
                    send! {
                        response_tx << service.db().cover_art(uri, kind).await
                    }
                },
                // Queue actions.
                Action::QueueGet { response_tx } => {
                    send! {
                        response_tx << service.queue().get().await
                    }
                },
                Action::QueueAdd { source, response_tx } => {
                    send! {
                        response_tx << service.queue().add(source).await
                    }
                },
                Action::QueueReplace { source, response_tx } => {
                    send! {
                        response_tx << service.queue().replace(source).await
                    }
                },
                Action::QueueClear { response_tx } => {
                    send! {
                        response_tx << service.queue().clear().await
                    }
                },
                Action::QueueRemove { id, response_tx } => {
                    send! {
                        response_tx << service.queue().remove(id).await
                    }
                },
                Action::QueueNext { response_tx } => {
                    send! {
                        response_tx << service.queue().next().await
                    }
                },
                Action::QueuePrev { response_tx } => {
                    send! {
                        response_tx << service.queue().prev().await
                    }
                },
                Action::QueueRepeat { state, response_tx } => {
                    send! {
                        response_tx << service.queue().repeat(state).await
                    }
                },
                Action::QueueConsume { state, response_tx } => {
                    send! {
                        response_tx << service.queue().consume(state).await
                    }
                },
                Action::QueueRandom { state, response_tx } => {
                    send! {
                        response_tx << service.queue().random(state).await
                    }
                },
                Action::QueueSingle { state, response_tx } => {
                    send! {
                        response_tx << service.queue().single(state).await
                    }
                },
                // Playlist actions.
                Action::PlaylistsGet { name, response_tx } => {
                    send! {
                        response_tx << service.playlists().get(name).await
                    }
                },
                Action::PlaylistsList { response_tx } => {
                    send! {
                        response_tx << service.playlists().list().await
                    }
                },
                Action::PlaylistsDelete { name, response_tx } => {
                    send! {
                        response_tx << service.playlists().delete(name).await
                    }
                },
                Action::PlaylistsDeleteSongs { name, positions, response_tx } => {
                    send! {
                        response_tx << service.playlists().delete_songs(name, positions).await
                    }
                },
                // Playback actions.
                Action::PlaybackPlay { id, response_tx } => {
                    send! {
                        response_tx << service.playback().play(id).await
                    }
                },
                Action::PlaybackToggle { response_tx } => {
                    send! {
                        response_tx << service.playback().toggle().await
                    }
                },
                Action::PlaybackStop { response_tx } => {
                    send! {
                        response_tx << service.playback().stop().await
                    }
                },
                Action::PlaybackSeek { time, response_tx } => {
                    send! {
                        response_tx << service.playback().seek(time).await
                    }
                },
                // Status actions.
                Action::StatusGet { response_tx } => {
                    send! {
                        response_tx << service.status().get().await
                    }
                },
                // Volume actions.
                Action::VolumeSet { value, response_tx } => {
                    send! {
                        response_tx << service.volume().set(value).await
                    }
                },
            }
        }

        async fn inner(&mut self, client: &mut Client) -> ControlFlow<()> {
            loop {
                let mut is_idling = false;

                tokio::select! {
                    changes = { client.idle(idle_input(), || is_idling = true) } => {
                        self.handle_changes(changes)?;
                    },
                    Some(action) = self.manager.action_rx.recv() => {
                        if is_idling {
                            let changes = client.noidle().await;

                            self.handle_changes(changes)?;
                        }

                        self.handle_action(client, action).await?;
                    }
                }
            }
        }

        pub async fn run(&mut self) {
            let mut ntries = 0usize;

            loop {
                match (self.manager.connect)().await {
                    Err(err) => {
                        tracing::warn!("connection failed: {err}, will retry in {} seconds", RECONNECT_TIMEOUT.as_secs());

                        time::sleep(RECONNECT_TIMEOUT).await;
                    }
                    Ok(mut client) => {
                        tracing::info!("connection established");

                        if ntries > 0 {
                            // Anything could've changed while we were disconnected.
                            self.manager.idle_tx.send(Ok(IDLE_SUBSYSTEMS.to_vec()))
                                .expect("expected to be able to send changes");
                        }

                        self.inner(&mut client).await;
                    }
                }

                ntries += 1;
            }
        }
    }
}
