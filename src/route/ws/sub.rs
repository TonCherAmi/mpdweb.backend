use std::time::Duration;

use tokio::time;

use crate::mpd;

pub struct Subscription {
    handle: mpd::Handle,
}

impl Subscription {
    pub fn new(handle: mpd::Handle) -> Self {
        Subscription { handle }
    }
}

pub enum Update {
    Db,
    Playlists,
    Status(mpd::Status),
    Queue(Vec<mpd::QueueItem>),
}

const STATUS_SUBSYSTEMS: &[mpd::Subsystem] = &[
    mpd::Subsystem::Volume,
    mpd::Subsystem::Player,
    mpd::Subsystem::Options,
    mpd::Subsystem::Queue,
];

const BATCH_SLEEP_DURATION: Duration = Duration::from_millis(10);

impl Subscription {
    pub async fn updates(&mut self) -> mpd::Result<Vec<Update>> {
        let mut changes = self.handle.changes().await?;

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

        if changes.contains(&mpd::Subsystem::Database) {
            updates.push(Update::Db);
        }

        if changes.contains(&mpd::Subsystem::Playlist) {
            updates.push(Update::Playlists);
        }

        if changes.iter().any(|it| STATUS_SUBSYSTEMS.contains(it)) {
            updates.push(Update::Status(self.handle.status().get().await?));
        }

        if changes.contains(&mpd::Subsystem::Queue) {
            updates.push(Update::Queue(self.handle.queue().get().await?));
        }

        Ok(updates)
    }
}
