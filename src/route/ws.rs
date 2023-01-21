use std::result;

use axum::Extension;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;

use crate::mpd;
use crate::route::ws::action::Action;
use crate::route::ws::proto::Out;
use crate::route::ws::proto::Request;
use crate::route::ws::proto::Status;
use crate::route::ws::proto::Update;
use crate::route::ws::proto::UpdateKind;
use crate::route::ws::sub::Subscription;

mod data;
mod proto;
mod sub;
mod action;

const UNKNOWN_ID: &str = "unknown";

type Result<T> = result::Result<T, String>;

struct Socket {
    inner: WebSocket,
}

impl Socket {
    fn new(socket: WebSocket) -> Self {
        Socket { inner: socket }
    }
}


impl Socket {
    async fn send<T: Into<Message>>(&mut self, payload: T) -> Result<()> {
        self.inner.send(payload.into()).await
            .map_err(|e| format!("ws connection closed: {e}"))
    }

    async fn recv(&mut self) -> Result<Option<String>> {
        loop {
            let Some(msg) = self.inner.recv().await else {
                return Ok(None);
            };

            match msg {
                Ok(Message::Text(msg)) => {
                    return Ok(Some(msg));
                },
                Ok(Message::Ping(_) | Message::Close(_)) => {
                    // These messages are handled automatically.
                },
                Ok(msg) => {
                    tracing::debug!("unsupported ws message type: {:?}", msg);
                },
                Err(err) => {
                    return Err(err.to_string());
                },
            }
        }
    }
}

struct Handle {
    inner: mpd::Handle,
}

impl Handle {
    fn new(handle: mpd::Handle) -> Self {
        Handle { inner: handle }
    }
}

impl Handle {
    async fn process(&self, action: Action) -> Status {
        let result = match action {
            // Database actions.
            Action::DbUpdate { uri } => {
                self.inner.db().update(uri).await
            },
            // Queue actions.
            Action::QueueAdd { source } => {
                self.inner.queue().add(source.into()).await
            },
            Action::QueueReplace { source } => {
                self.inner.queue().replace(source.into()).await
            },
            Action::QueueClear => {
                self.inner.queue().clear().await
            },
            Action::QueueRemove { id } => {
                self.inner.queue().remove(id).await
            },
            Action::QueueNext => {
                self.inner.queue().next().await
            },
            Action::QueuePrev => {
                self.inner.queue().prev().await
            },
            Action::QueueRepeat { state } => {
                self.inner.queue().repeat(state).await
            },
            Action::QueueConsume { state } => {
                self.inner.queue().consume(state.into()).await
            },
            Action::QueueRandom { state } => {
                self.inner.queue().random(state).await
            },
            Action::QueueSingle { state } => {
                self.inner.queue().single(state.into()).await
            },
            // Playback actions.
            Action::PlaybackPlay { id } => {
                self.inner.playback().play(id).await
            },
            Action::PlaybackToggle => {
                self.inner.playback().toggle().await
            },
            Action::PlaybackStop => {
                self.inner.playback().stop().await
            },
            Action::PlaybackSeek { time } => {
                self.inner.playback().seek(time).await
            },
            // Volume actions.
            Action::VolumeSet { value } => {
                self.inner.volume().set(value).await
            },
        };

        result.map_or_else(Into::into, |_| Status::success())
    }

    async fn initial_update(&self) -> Update {
        match (self.inner.status().get().await, self.inner.queue().get().await) {
            (Ok(status), Ok(queue)) => {
                Update::from_data(
                    vec![
                        UpdateKind::Db,
                        UpdateKind::Playlists,
                        UpdateKind::Status(status.into()),
                        UpdateKind::Queue(queue.into_iter().map(Into::into).collect()),
                    ]
                )
            },
            (Err(err), _) | (_, Err(err)) => {
                Update::from_err(err)
            },
        }
    }
}

impl From<mpd::Result<Vec<sub::Update>>> for Update {
    fn from(updates: mpd::Result<Vec<sub::Update>>) -> Self {
        match updates {
            Ok(updates) => {
                Update::from_data(updates.into_iter().map(Into::into).collect())
            },
            Err(err) => {
                Update::from_err(err)
            },
        }
    }
}

async fn handle_upgrade(socket: WebSocket, handle: mpd::Handle) -> Result<()> {
    let mut socket = Socket::new(socket);

    let mut sub = Subscription::new(handle.clone());

    let handle = Handle::new(handle);

    socket.send(Out::update(handle.initial_update().await)).await?;

    loop {
        tokio::select! {
            updates = sub.updates() => {
                socket.send(Out::update(updates.into())).await?;
            },
            msg = socket.recv() => {
                let Ok(Some(msg)) = msg else {
                    return msg.map(|_| ());
                };

                match msg.parse() {
                    Ok(Request { id, content: action }) => {
                        let status = handle.process(action).await;

                        socket.send(Out::response(id, status)).await?;
                    },
                    Err(err) => {
                        socket.send(
                            Out::response(
                                UNKNOWN_ID.to_owned(),
                                Status::new(
                                    Status::PARSE_ERR_CODE,
                                    Some(format!("failed to parse request: {err}")),
                                ),
                            )
                        ).await?;
                    },
                }
            }
        }
    }
}

#[tracing::instrument(skip(ws, handle), level = "debug")]
pub async fn websocket(
    ws: WebSocketUpgrade,
    Extension(handle): Extension<mpd::Handle>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        match handle_upgrade(socket, handle).await {
            Ok(_) => tracing::debug!("connection closed"),
            Err(err) => tracing::debug!("connection closed with error: {err}"),
        };
    })
}
