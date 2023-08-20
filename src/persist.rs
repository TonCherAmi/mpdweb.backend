use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use sqlx::ConnectOptions;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::SqlitePool;
use sqlx::Type;
use time::Duration;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use tracing::log::LevelFilter;

use crate::convert::IntoResult;
use crate::mpd::DbTags;
use crate::persist::repo::CreatePlaybackHistoryEventRow;
use crate::persist::repo::CreatePlaybackHistoryMetadataRow;
use crate::persist::repo::IdRow;
pub use crate::persist::repo::PlaybackHistoryEventKind;
use crate::persist::repo::PlaybackHistoryEventRow;
use crate::persist::repo::PlaybackHistoryMetadataRow;
pub use crate::persist::repo::PlaybackHistoryPlayId;
use crate::persist::repo::Pool;
use crate::persist::result::Result;
pub use crate::persist::error::Error;

mod repo;
mod error;
mod result;

#[derive(Type, Debug)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventKind {
    Start,
    Interrupt,
}

pub struct PlaybackHistoryEventHandle<'a> {
    inner: &'a Handle,
}

pub struct CreatePlaybackHistoryEvent {
    pub play_id: PlaybackHistoryPlayId,
    pub elapsed: Duration,
    pub kind: PlaybackHistoryEventKind,
    pub recorded_at: OffsetDateTime,
}

#[derive(Debug)]
pub struct PlaybackHistoryEvent {
    pub play_id: i64,
    pub elapsed: Duration,
    pub kind: PlaybackHistoryEventKind,
    pub recorded_at: OffsetDateTime,
}

fn format_iso8601(datetime: OffsetDateTime) -> std::result::Result<String, String> {
    datetime.format(&Iso8601::DEFAULT)
        .map_err(|err| format!("failed to format date: {err}"))
}

impl TryFrom<CreatePlaybackHistoryEvent> for CreatePlaybackHistoryEventRow {
    type Error = String;

    fn try_from(create: CreatePlaybackHistoryEvent) -> std::result::Result<Self, Self::Error> {
        CreatePlaybackHistoryEventRow {
            play_id: create.play_id,
            elapsed: create.elapsed.as_seconds_f64(),
            kind: create.kind,
            recorded_at: format_iso8601(create.recorded_at)?,
        }.into_ok()
    }
}

impl TryFrom<PlaybackHistoryEventRow> for PlaybackHistoryEvent {
    type Error = String;

    fn try_from(row: PlaybackHistoryEventRow) -> std::result::Result<Self, Self::Error> {
        PlaybackHistoryEvent {
            play_id: row.play_id,
            elapsed: Duration::seconds_f64(row.elapsed),
            kind: row.kind,
            recorded_at: OffsetDateTime::parse(&row.recorded_at, &Iso8601::DEFAULT)
                .map_err(|err| format!("failed to parse recorded_at timestamp: {err}"))?,
        }.into_ok()
    }
}

impl<'a> PlaybackHistoryEventHandle<'a> {
    pub async fn get_latest(&self) -> Result<Option<PlaybackHistoryEvent>> {
        let mut repo = self.inner.pool.acquire().await?;

        let result = repo.playback_history_event()
            .get_latest()
            .await?
            .map(TryInto::try_into)
            .transpose()?;

        Ok(result)
    }

    pub async fn get_all(&self, from: Option<OffsetDateTime>, to: Option<OffsetDateTime>) -> Result<Vec<PlaybackHistoryEvent>> {
        let mut repo = self.inner.pool.acquire().await?;

        let from = from.map(format_iso8601).transpose()?;
        let to = to.map(format_iso8601).transpose()?;

        let result = repo.playback_history_event()
            .get_all(from.as_deref(), to.as_deref())
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(result)
    }

    pub async fn create(&self, create: CreatePlaybackHistoryEvent) -> Result<PlaybackHistoryEvent> {
        let mut repo = self.inner.pool.begin().await?;

        let IdRow { id } = repo.playback_history_event()
            .create(create.try_into()?)
            .await?;

        let result = repo.playback_history_event()
            .get_by_id(id)
            .await?
            .try_into()?;

        repo.commit().await?;

        Ok(result)
    }

    pub async fn create_all(&self, create: Vec<CreatePlaybackHistoryEvent>) -> Result<Vec<PlaybackHistoryEvent>> {
        let mut repo = self.inner.pool.begin().await?;

        let ids = repo.playback_history_event().create_all(
            create.into_iter()
                .map(TryInto::try_into)
                .collect::<std::result::Result<_, _>>()?
        ).await?.into_iter().map(|IdRow { id }| id).collect::<Vec<_>>();

        let result = repo.playback_history_event()
            .get_all_by_id(&ids)
            .await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<std::result::Result<Vec<_>, String>>()?;

        repo.commit().await?;

        Ok(result)
    }
}

pub struct PlaybackHistoryMetadataHandle<'a> {
    inner: &'a Handle,
}

pub struct CreatePlaybackHistoryMetadata {
    pub play_id: PlaybackHistoryPlayId,
    pub playlist_id: i64,
    pub uri: String,
    pub duration: Duration,
    pub tags: DbTags,
}

#[derive(Debug)]
pub struct PlaybackHistoryMetadata {
    pub play_id: PlaybackHistoryPlayId,
    pub playlist_id: i64,
    pub uri: String,
    pub duration: Duration,
    pub tags: DbTags,
}

impl From<CreatePlaybackHistoryMetadata> for Vec<CreatePlaybackHistoryMetadataRow> {
    fn from(create: CreatePlaybackHistoryMetadata) -> Self {
        let tags = |key: String, vec: Vec<String>| -> Vec<CreatePlaybackHistoryMetadataRow> {
            vec.into_iter().map(|value| {
                CreatePlaybackHistoryMetadataRow {
                    play_id: create.play_id,
                    key: key.clone(),
                    value,
                }
            }).collect()
        };

        let titles = tags("title".to_owned(), create.tags.titles);
        let artists = tags("artist".to_owned(), create.tags.artists);
        let albums = tags("album".to_owned(), create.tags.albums);

        vec![
            vec![
                CreatePlaybackHistoryMetadataRow {
                    play_id: create.play_id,
                    key: "uri".to_owned(),
                    value: create.uri,
                },
                CreatePlaybackHistoryMetadataRow {
                    play_id: create.play_id,
                    key: "playlist_id".to_owned(),
                    value: create.playlist_id.to_string(),
                },
                CreatePlaybackHistoryMetadataRow {
                    play_id: create.play_id,
                    key: "duration".to_owned(),
                    value: create.duration.as_seconds_f64().to_string(),
                },
            ],
            titles,
            artists,
            albums,
        ].concat()
    }
}

impl TryFrom<Vec<PlaybackHistoryMetadataRow>> for PlaybackHistoryMetadata {
    type Error = String;

    fn try_from(rows: Vec<PlaybackHistoryMetadataRow>) -> std::result::Result<PlaybackHistoryMetadata, Self::Error> {
        let play_id = rows.first().ok_or("rows are empty")?.play_id;

        let mut map: HashMap<String, Vec<_>> = HashMap::new();

        for row in rows {
            map.entry(row.key)
                .or_default()
                .push(row.value);
        }

        PlaybackHistoryMetadata {
            play_id,
            playlist_id: map.remove("playlist_id").ok_or("playlist_id is missing")?
                .swap_remove(0).parse().map_err(|e| format!("cannot parse playlist_id as i64: {e}"))?,
            uri: map.remove("uri").ok_or("uri is missing")?.swap_remove(0),
            duration: map.remove("duration").ok_or("duration is missing")?
                .swap_remove(0).parse().map(Duration::seconds_f64).map_err(|e| format!("cannot parse duration as f64: {e}"))?,
            tags: DbTags {
                titles: map.remove("title").unwrap_or_default(),
                artists: map.remove("artist").unwrap_or_default(),
                albums: map.remove("album").unwrap_or_default(),
            }
        }.into_ok()
    }
}

impl<'a> PlaybackHistoryMetadataHandle<'a> {
    pub async fn create(&mut self, create: CreatePlaybackHistoryMetadata) -> Result<PlaybackHistoryMetadata> {
        let mut repo = self.inner.pool.begin().await?;

        let play_id = create.play_id;

        repo.playback_history_metadata()
            .create_all(create.into())
            .await?;

        let result = repo.playback_history_metadata()
            .get_by_play_id(play_id)
            .await?
            .try_into()?;

        repo.commit().await?;

        Ok(result)
    }

    pub async fn get_by_play_id(&mut self, play_id: PlaybackHistoryPlayId) -> Result<PlaybackHistoryMetadata> {
        let mut repo = self.inner.pool.acquire().await?;

        let result = repo.playback_history_metadata()
            .get_by_play_id(play_id)
            .await?
            .try_into()?;

        Ok(result)
    }

    pub async fn get_all_by_play_id(
        &mut self,
        play_ids: &[PlaybackHistoryPlayId]
    ) -> Result<Vec<PlaybackHistoryMetadata>> {
        let mut repo = self.inner.pool.acquire().await?;

        let rows = repo.playback_history_metadata()
            .get_all_by_play_id(play_ids)
            .await?;

        let mut map: HashMap<_, Vec<_>> = HashMap::new();

        for row in rows {
            map.entry(row.play_id)
                .or_default()
                .push(row);
        }

        map.into_values()
            .map(TryInto::try_into)
            .collect::<std::result::Result<Vec<_>, _>>()?
            .into_ok()
    }
}

#[derive(Clone)]
pub struct Handle {
    pool: Pool,
}

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn init(path: PathBuf) -> Result<Handle> {
    let mut options = SqliteConnectOptions::from_str(format!("sqlite://{}", path.display()).as_str())?
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true);

    options.log_statements(LevelFilter::Warn);

    let pool = SqlitePool::connect_with(options).await?;

    MIGRATOR.run(&pool).await?;

    Handle::new(pool).into_ok()
}

impl Handle {
    pub fn new(pool: SqlitePool) -> Self {
        Handle { pool: Pool::new(pool) }
    }
}

impl Handle {
    pub fn playback_history_event(&self) -> PlaybackHistoryEventHandle {
        PlaybackHistoryEventHandle { inner: self }
    }

    pub fn playback_history_metadata(&self) -> PlaybackHistoryMetadataHandle {
        PlaybackHistoryMetadataHandle { inner: self }
    }
}
