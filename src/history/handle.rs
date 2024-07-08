use std::collections::HashMap;

use time::Duration;
use time::OffsetDateTime;

use crate::convert::IntoOption;
use crate::convert::IntoResult;
use crate::mpd::DbTags;
use crate::persist;
use crate::persist::PlaybackHistoryEvent;
use crate::persist::PlaybackHistoryMetadata;
use crate::persist::PlaybackHistoryPlayId;

#[derive(Clone)]
pub struct Handle {
    inner: persist::Handle,
}

impl Handle {
    pub fn new(persistence_handle: persist::Handle) -> Self {
        Handle { inner: persistence_handle }
    }
}

pub struct HistoryEntry {
    pub id: i64,
    pub uri: String,
    pub tags: DbTags,
    pub _duration: Duration,
    pub recorded_at: OffsetDateTime,
}

impl From<(PlaybackHistoryEvent, PlaybackHistoryMetadata)> for HistoryEntry {
    fn from((event, metadata): (PlaybackHistoryEvent, PlaybackHistoryMetadata)) -> Self {
        HistoryEntry {
            id: event.play_id,
            uri: metadata.uri,
            tags: metadata.tags,
            _duration: metadata.duration,
            recorded_at: event.recorded_at,
        }
    }
}

impl Handle {
    pub async fn get(
        &self,
        from: Option<OffsetDateTime>,
        to: Option<OffsetDateTime>
    ) -> Result<Vec<HistoryEntry>, String> {
        let mut events = self.inner.playback_history_event()
            .get_all(from, to).await?;

        events.dedup_by_key(|event| event.play_id);

        let metadata = self.inner.playback_history_metadata()
            .get_all_by_play_id(&events.iter().map(|x| x.play_id).collect::<Vec<_>>())
            .await?;

        let mut map: HashMap<PlaybackHistoryPlayId, PlaybackHistoryMetadata> = HashMap::new();

        for entry in metadata {
            map.insert(entry.play_id, entry);
        }

        events.into_iter()
            .filter_map(|x| {
                let play_id = x.play_id;

                HistoryEntry::from((x, map.remove(&play_id)?)).into_some()
            })
            .collect::<Vec<_>>()
            .into_ok()
    }
}
