use std::cmp;
use std::ops::Add;
use std::ops::Sub;

use time::Duration;
use time::OffsetDateTime;

use crate::convert::IntoOption;
use crate::convert::IntoResult;
use crate::convert::IntoVec;
use crate::history::keeper::result::Result;
use crate::mpd;
use crate::mpd::PlaybackState;
use crate::mpd::QueueItem;
use crate::mpd::SongStatus;
use crate::mpd::Status;
use crate::mpd::Update;
use crate::persist;
use crate::persist::CreatePlaybackHistoryEvent;
use crate::persist::CreatePlaybackHistoryMetadata;
use crate::persist::PlaybackHistoryEvent;
use crate::persist::PlaybackHistoryEventKind;
use crate::persist::PlaybackHistoryMetadata;
use crate::persist::PlaybackHistoryPlayId;

mod error;
mod result;

#[derive(Debug)]
enum StatusDiff<'a> {
    PlaybackStart(&'a SongStatus),
    PlaybackPause,
    PlaybackResume,
    PlaybackStop,
    SongChange(&'a SongStatus),
    Other(&'a SongStatus),
}

fn diff<'a>(old: &'a Status, new: &'a Status) -> Option<StatusDiff<'a>> {
    tracing::debug!(?old, ?new, "diff between");

    let diff = match (&old.song, &new.song) {
        (None, None) => {
            return None;
        },
        (None, Some(song_status)) => {
            StatusDiff::PlaybackStart(song_status)
        },
        (Some(_), None) => {
            StatusDiff::PlaybackStop
        },
        (Some(SongStatus { id: old_id, .. }),
            Some(song_status @ SongStatus { id: new_id, .. }),
        ) if old_id != new_id => {
            StatusDiff::SongChange(song_status)
        },
        (Some(_), Some(song_status)) => {
            match (&old.state, &new.state) {
                (PlaybackState::Paused, PlaybackState::Playing) => StatusDiff::PlaybackResume,
                (PlaybackState::Playing, PlaybackState::Paused) => StatusDiff::PlaybackPause,
                _ => StatusDiff::Other(song_status),
            }
        },
    };

    Some(diff)
}

const INITIAL_PLAY_ID: i64 = 1;

impl PlaybackHistoryEvent {
    fn elapsed_now(&self, duration: Duration) -> Duration {
        cmp::min(
            duration,
            OffsetDateTime::now_utc()
                .sub(self.recorded_at)
                .add(self.elapsed),
        )
    }
}

fn filter(updates: Vec<Update>) -> (Option<Status>, Option<Vec<QueueItem>>) {
    let mut status = None;
    let mut queue = None;

    // TODO: Handle disconnect.
    for update in updates {
        match update {
            Update::Status(new_status) => {
                status = Some(new_status);
            },
            Update::Queue(new_queue) => {
                queue = Some(new_queue);
            },
            Update::Db | Update::Playlists => {
                // We don't need these.
            },
        }
    }

    (status, queue)
}

#[derive(Debug)]
struct State {
    pub event: PlaybackHistoryEvent,
    pub metadata: PlaybackHistoryMetadata,
}

impl State {
    async fn last(persistence_handle: &persist::Handle) -> Result<Option<Self>> {
        let event = persistence_handle.playback_history_event()
            .get_latest()
            .await?;

        let Some(event) = event else {
            return Ok(None);
        };

        let play_id = event.play_id;

        let metadata = persistence_handle.playback_history_metadata()
            .get_by_play_id(play_id)
            .await?;

        Some(State { event, metadata }).into_ok()
    }
}

impl CreatePlaybackHistoryEvent {
    fn new(play_id: PlaybackHistoryPlayId, elapsed: Duration, kind: PlaybackHistoryEventKind) -> Self {
        CreatePlaybackHistoryEvent {
            play_id,
            elapsed,
            kind,
            recorded_at: OffsetDateTime::now_utc(),
        }
    }

    fn from_state(state: &State, new_kind: PlaybackHistoryEventKind) -> Self {
        CreatePlaybackHistoryEvent::new(
            state.event.play_id,
            state.event.elapsed_now(state.metadata.duration),
            new_kind,
        )
    }
}

impl CreatePlaybackHistoryMetadata {
    fn new(play_id: PlaybackHistoryPlayId, song: &QueueItem) -> Self {
        CreatePlaybackHistoryMetadata {
            play_id,
            playlist_id: song.id,
            uri: song.uri.clone(),
            duration: song.duration,
            tags: song.tags.clone(),
        }
    }
}

async fn process_initial(
    persistence_handle: &persist::Handle,
    state: Option<State>,
    status: &Status,
    queue: &[QueueItem],
) -> Result<Option<State>> {
    let initial = status.song.as_ref().and_then(|song_status| {
        queue.get(song_status.position as usize)
            .map(|song| (song_status, song))
    });

    let Some((song_status, song)) = initial else {
        return Ok(state);
    };

    async fn is_matching_play(p: &persist::Handle, play_id: PlaybackHistoryPlayId, song: &QueueItem) -> Result<bool> {
        let metadata = p.playback_history_metadata().get_by_play_id(play_id).await?;

        Ok(metadata.playlist_id == song.id && metadata.uri == song.uri)
    }

    match state {
        None => {
            let play_id = INITIAL_PLAY_ID;

            let metadata = persistence_handle.playback_history_metadata().create(
                CreatePlaybackHistoryMetadata::new(play_id, song)
            ).await?;

            let event = persistence_handle.playback_history_event().create(
                CreatePlaybackHistoryEvent::new(
                    play_id,
                    song_status.elapsed,
                    PlaybackHistoryEventKind::Start,
                )
            ).await?;

            Some(State { event, metadata }).into_ok()
        },
        Some(state) if is_matching_play(persistence_handle, state.event.play_id, song).await? => {
            let is_playback_uninterrupted = (OffsetDateTime::now_utc() - state.event.recorded_at)
                - (song_status.elapsed - state.event.elapsed)
                > Duration::seconds(1);

            if is_playback_uninterrupted || state.event.kind == PlaybackHistoryEventKind::Stop {
                return Some(state).into_ok();
            }

            persistence_handle.playback_history_event().create(
                CreatePlaybackHistoryEvent::new(
                    state.event.play_id,
                    state.event.elapsed,
                    PlaybackHistoryEventKind::Stop,
                )
            ).await?;

            let new_play_id = state.event.play_id + 1;

            let metadata = persistence_handle.playback_history_metadata().create(
                CreatePlaybackHistoryMetadata::new(new_play_id, song)
            ).await?;

            let event = persistence_handle.playback_history_event().create(
                CreatePlaybackHistoryEvent::new(
                    new_play_id,
                    song_status.elapsed,
                    PlaybackHistoryEventKind::Start,
                )
            ).await?;

            Some(State { event, metadata }).into_ok()
        },
        Some(state) => {
            let new_play_id = state.event.play_id + 1;

            let metadata = persistence_handle.playback_history_metadata().create(
                CreatePlaybackHistoryMetadata::new(new_play_id, song)
            ).await?;

            let event = persistence_handle.playback_history_event().create(
                CreatePlaybackHistoryEvent::new(
                    new_play_id,
                    song_status.elapsed,
                    PlaybackHistoryEventKind::Start,
                )
            ).await?;

            Some(State { event, metadata }).into_ok()
        }
    }
}

fn process_diff(
    diff: StatusDiff,
    state: Option<&State>,
    queue: &[QueueItem],
) -> Option<(Vec<CreatePlaybackHistoryEvent>, Option<CreatePlaybackHistoryMetadata>)> {
    match (diff, state) {
        (StatusDiff::PlaybackStart(song_status), state) => {
            let Some(song) = queue.get(song_status.position as usize) else {
                return None;
            };

            let play_id = state.map_or(INITIAL_PLAY_ID, |it| it.event.play_id + 1);

            let events = CreatePlaybackHistoryEvent::new(
                play_id,
                song_status.elapsed,
                PlaybackHistoryEventKind::Start,
            ).into_vec();

            let metadata = CreatePlaybackHistoryMetadata::new(play_id, song);

            (events, Some(metadata)).into_some()
        },
        (diff @ StatusDiff::PlaybackPause | diff @ StatusDiff::PlaybackResume | diff @ StatusDiff::PlaybackStop, Some(state)) => {
            let kind = match diff {
                StatusDiff::PlaybackPause => PlaybackHistoryEventKind::Pause,
                StatusDiff::PlaybackResume => PlaybackHistoryEventKind::Resume,
                StatusDiff::PlaybackStop => PlaybackHistoryEventKind::Stop,
                _ => unreachable!(),
            };

            let events = CreatePlaybackHistoryEvent::from_state(state, kind).into_vec();

            (events, None).into_some()
        },
        (StatusDiff::SongChange(song_status), Some(state)) => {
            let Some(song) = queue.get(song_status.position as usize) else {
                return None;
            };

            let new_play_id = state.event.play_id + 1;

            let events = vec![
                CreatePlaybackHistoryEvent::from_state(
                    state,
                    PlaybackHistoryEventKind::Stop,
                ),
                CreatePlaybackHistoryEvent::new(
                    new_play_id,
                    song_status.elapsed,
                    PlaybackHistoryEventKind::Start,
                ),
            ];

            let metadata = CreatePlaybackHistoryMetadata::new(new_play_id, song);

            (events, Some(metadata)).into_some()
        },
        (StatusDiff::Other(song_status), Some(state)) => {
            tracing::debug!(
                "elapsed_now: {}, song_status.elapsed: {}",
                state.event.elapsed_now(state.metadata.duration),
                song_status.elapsed
            );

            if (state.event.elapsed_now(state.metadata.duration) - song_status.elapsed) < Duration::seconds(1) {
                return None;
            }

            let events = CreatePlaybackHistoryEvent::new(
                state.event.play_id,
                song_status.elapsed,
                PlaybackHistoryEventKind::Seek,
            ).into_vec();

            (events, None).into_some()
        },
        (diff, state) => {
            tracing::warn!(?diff, ?state, "unexpected combination of status diff and last event");

            None
        }
    }
}

async fn inner(
    handle: &mpd::Handle,
    sub_handle: &mut mpd::SubscriptionHandle,
    persistence_handle: &persist::Handle
) -> Result<()> {
    let mut queue = handle.queue().get().await?;
    let mut status = handle.status().get().await?;

    let state = State::last(persistence_handle).await?;

    let mut state = process_initial(persistence_handle, state, &status, &queue).await?;

    // TODO: Handle interrupt / ongoing playback w/o interrupt (power cord yanked) type stuff.
    loop {
        let (Some(new_status), new_queue) = filter(sub_handle.updates().await?) else {
            continue;
        };

        if let Some(new_queue) = new_queue {
            queue = new_queue;
        }

        let Some(diff) = diff(&status, &new_status) else {
            tracing::debug!("diff is none");

            continue;
        };

        tracing::debug!(?diff);

        let Some((events, metadata)) = process_diff(diff, state.as_ref(), &queue) else {
            continue;
        };

        status = new_status;

        let metadata = if let Some(metadata) = metadata {
            persistence_handle.playback_history_metadata().create(metadata).await?.into_some()
        } else {
            None
        };

        let mut events = persistence_handle.playback_history_event()
            .create_all(events)
            .await?;

        state = State {
            event: events.swap_remove(events.len() - 1),
            metadata: metadata.unwrap_or(state.unwrap().metadata),
        }.into_some();
    }
}

pub fn run(handle: mpd::Handle, mut sub_handle: mpd::SubscriptionHandle, persistence_handle: persist::Handle) {
    tokio::spawn(async move {
        match inner(&handle, &mut sub_handle, &persistence_handle).await {
            Ok(_) => {
                tracing::debug!("inner exited without error");
            },
            // TODO: Handle MPD disconnect.
            Err(err) => {
                tracing::error!("inner exited with error: {err}");
            },
        }
    });
}
