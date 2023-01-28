use tokio::sync::watch;

use crate::mpd::data::QueueItem;
use crate::mpd::data::Status;
use crate::mpd::handle::Handle;
use crate::mpd::result::Result;

#[derive(Clone)]
pub struct SubscriptionHandle {
    updates_rx: watch::Receiver<Result<Vec<Update>>>
}

impl SubscriptionHandle {
    pub fn new(handle: Handle) -> Self {
        let (updates_tx, updates_rx) = watch::channel(Ok(Vec::new()));

        let recv_loop = recv::RecvLoop::new(handle, updates_tx);

        tokio::spawn(recv::run(recv_loop));

        SubscriptionHandle { updates_rx }
    }
}

impl SubscriptionHandle {
    pub async fn updates(&mut self) -> Result<Vec<Update>> {
        self.updates_rx.changed().await.expect("updates sender is dropped");

        self.updates_rx.borrow_and_update().clone()
    }
}

#[derive(Debug, Clone)]
pub enum Update {
    Db,
    Playlists,
    Status(Status),
    Queue(Vec<QueueItem>),
}

impl SubscriptionHandle {
}

mod recv {
    use std::time::Duration;

    use tokio::sync::watch;
    use tokio::time;

    use crate::mpd::data::Subsystem;
    use crate::mpd::handle::Handle;
    use crate::mpd::result::Result;

    use super::Update;

    const STATUS_SUBSYSTEMS: &[Subsystem] = &[
        Subsystem::Volume,
        Subsystem::Player,
        Subsystem::Options,
        Subsystem::Queue,
    ];

    const BATCH_SLEEP_DURATION: Duration = Duration::from_millis(10);

    pub struct RecvLoop {
        handle: Handle,
        updates_tx: watch::Sender<Result<Vec<Update>>>,
    }

    impl RecvLoop {
        pub fn new(handle: Handle, updates_tx: watch::Sender<Result<Vec<Update>>>) -> Self {
            RecvLoop { handle, updates_tx }
        }
    }

    impl RecvLoop {
        async fn updates(&mut self) -> Result<Vec<Update>> {
            let mut changes = self.handle.changes().await?;

            tracing::debug!(?changes, "received");

            // Try to batch changes together.
            tokio::select! {
                other = self.handle.changes() => {
                    changes.append(&mut other?);
                },
                _ = time::sleep(BATCH_SLEEP_DURATION) => {
                    // do nothing
                }
            }

            let mut updates = Vec::new();

            if changes.contains(&Subsystem::Database) {
                updates.push(Update::Db);
            }

            if changes.contains(&Subsystem::Playlist) {
                updates.push(Update::Playlists);
            }

            if changes.iter().any(|it| STATUS_SUBSYSTEMS.contains(it)) {
                updates.push(Update::Status(self.handle.status().get().await?));
            }

            if changes.contains(&Subsystem::Queue) {
                updates.push(Update::Queue(self.handle.queue().get().await?));
            }

            Ok(updates)
        }
    }

    pub async fn run(mut recv_loop: RecvLoop) {
        loop {
            let updates = recv_loop.updates().await;

            recv_loop.updates_tx.send(updates)
                .expect("expected to be able to propagate updates");
        }
    }
}
