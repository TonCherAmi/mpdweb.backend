use sqlx::FromRow;
use sqlx::query;
use sqlx::pool::PoolConnection;
use sqlx::query_as;
use sqlx::QueryBuilder;
use sqlx::Sqlite;
use sqlx::SqliteConnection;
use sqlx::SqlitePool;
use sqlx::Type;

use crate::persist::result::Result;

#[derive(FromRow)]
pub struct IdRow<T> {
    pub id: T,
}

// <editor-fold desc="Playback History Event">

pub struct PlaybackHistoryEventRepository<'c> {
    inner: &'c mut SqliteConnection,
}

#[derive(Type, Debug, Copy, Clone, PartialEq)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaybackHistoryEventKind {
    Start,
    Pause,
    Resume,
    Stop,
    Seek,
    Interrupt,
}

pub type PlaybackHistoryPlayId = i64;

pub type PlaybackHistoryEventId = i64;

#[derive(FromRow)]
pub struct PlaybackHistoryEventRow {
    pub play_id: PlaybackHistoryPlayId,
    pub elapsed: f64,
    pub kind: PlaybackHistoryEventKind,
    pub recorded_at: String,
}

pub struct CreatePlaybackHistoryEventRow {
    pub play_id: PlaybackHistoryPlayId,
    pub elapsed: f64,
    pub kind: PlaybackHistoryEventKind,
    pub recorded_at: String,
}

impl<'c> PlaybackHistoryEventRepository<'c> {
    pub async fn get_by_id(&mut self, id: PlaybackHistoryEventId) -> Result<PlaybackHistoryEventRow> {
        let sql = /* language=sql */ r#"
            SELECT "play_id", "elapsed", "kind", "recorded_at"
            FROM "playback_history_events"
            WHERE "id" = ?
        "#;

        query_as(sql)
            .bind(id)
            .fetch_one(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn get_latest(&mut self) -> Result<Option<PlaybackHistoryEventRow>> {
        let sql = /* language=sql */ r#"
            SELECT "play_id", "elapsed", "kind", "recorded_at"
            FROM "playback_history_events"
            ORDER BY "recorded_at" DESC
            LIMIT 1
        "#;

        query_as(sql)
            .fetch_optional(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn get_all(&mut self, from: Option<&str>, to: Option<&str>) -> Result<Vec<PlaybackHistoryEventRow>> {
        let sql = /* language=sql */ r#"
            SELECT "play_id", "elapsed", "kind", "recorded_at"
            FROM "playback_history_events"
            WHERE (?1 IS NULL OR "recorded_at" >= ?1)
              AND (?2 IS NULL OR "recorded_at" < ?2)
            ORDER BY "recorded_at" DESC
        "#;

        query_as(sql)
            .bind(from)
            .bind(to)
            .fetch_all(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn get_all_by_id(&mut self, ids: &[PlaybackHistoryEventId]) -> Result<Vec<PlaybackHistoryEventRow>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let mut builder = QueryBuilder::new(r#"
            SELECT "play_id", "elapsed", "kind", "recorded_at"
            FROM "playback_history_events"
            WHERE "id" IN (
        "#);

        let mut separated = builder.separated(", ");

        for id in ids {
            separated.push_bind(id);
        }

        separated.push_unseparated(")");

        builder.build_query_as()
            .fetch_all(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn create(&mut self, create: CreatePlaybackHistoryEventRow) -> Result<IdRow<PlaybackHistoryEventId>> {
        let sql = /* language=sql */ r#"
            INSERT INTO "playback_history_events"
            ("play_id", "elapsed", "kind", "recorded_at")
            VALUES
            (?, ?, ?, ?)
            RETURNING "id"
        "#;

        query_as(sql)
            .bind(create.play_id)
            .bind(create.elapsed)
            .bind(create.kind)
            .bind(create.recorded_at)
            .fetch_one(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn create_all(&mut self, create: Vec<CreatePlaybackHistoryEventRow>) -> Result<Vec<IdRow<PlaybackHistoryEventId>>> {
        if create.is_empty() {
            return Ok(vec![]);
        }

        let mut builder = QueryBuilder::new(r#"
            INSERT INTO "playback_history_events" ("play_id", "elapsed", "kind", "recorded_at")
        "#);

        builder.push_values(create, |mut builder, create| {
            builder.push_bind(create.play_id)
                .push_bind(create.elapsed)
                .push_bind(create.kind)
                .push_bind(create.recorded_at);
        });

        builder.push(r#"RETURNING "id""#);

        builder.build_query_as()
            .fetch_all(&mut *self.inner)
            .await
            .map_err(Into::into)
    }
}

// </editor-fold>

// <editor-fold desc="Playback History Metadata">

pub struct PlaybackHistoryMetadataRepository<'c> {
    inner: &'c mut SqliteConnection,
}

#[derive(FromRow)]
pub struct PlaybackHistoryMetadataRow {
    pub play_id: PlaybackHistoryPlayId,
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct CreatePlaybackHistoryMetadataRow {
    pub play_id: PlaybackHistoryPlayId,
    pub key: String,
    pub value: String,
}

impl<'c> PlaybackHistoryMetadataRepository<'c> {
    pub async fn create_all(&mut self, create: Vec<CreatePlaybackHistoryMetadataRow>) -> Result<()> {
        if create.is_empty() {
            return Ok(());
        }

        let mut builder = QueryBuilder::new(r#"
            INSERT INTO "playback_history_metadata" ("play_id", "key", "value")
        "#);

        builder.push_values(create, |mut builder, create| {
            builder.push_bind(create.play_id)
                .push_bind(create.key)
                .push_bind(create.value);
        });

        builder.build()
            .execute(&mut *self.inner)
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    pub async fn get_by_play_id(&mut self, play_id: PlaybackHistoryPlayId) -> Result<Vec<PlaybackHistoryMetadataRow>> {
        let sql = /* language=sql */ r#"
            SELECT "play_id", "key", "value"
            FROM "playback_history_metadata"
            WHERE "play_id" = ?
        "#;

        query_as(sql)
            .bind(play_id)
            .fetch_all(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn get_all_by_play_id(&mut self, play_ids: &[PlaybackHistoryPlayId]) -> Result<Vec<PlaybackHistoryMetadataRow>> {
        let mut builder = QueryBuilder::new(r#"
            SELECT "play_id", "key", "value"
            FROM "playback_history_metadata"
            WHERE "play_id" IN (
        "#);

        let mut separated = builder.separated(", ");

        for play_id in play_ids {
            separated.push_bind(play_id);
        }

        separated.push_unseparated(")");

        builder.push(r#"ORDER BY "play_id""#);

        builder.build_query_as()
            .fetch_all(&mut *self.inner)
            .await
            .map_err(Into::into)
    }
}

// </editor-fold>

// <editor-fold desc="Db Item Label">

pub struct DbItemLabelRepository<'c> {
    inner: &'c mut SqliteConnection,
}

pub type DbItemLabelId = i64;

#[derive(FromRow)]
pub struct DbItemLabelRow {
    pub id: DbItemLabelId,
    pub uri: String,
    pub scope: String,
    pub key: String,
    pub value: String,
    pub created_at: String,
}

#[derive(Clone)]
pub struct CreateDbItemLabelRow {
    pub uri: String,
    pub scope: String,
    pub key: String,
    pub value: String,
    pub created_at: String,
}

impl<'c> DbItemLabelRepository<'c> {
    pub async fn create(&mut self, create: CreateDbItemLabelRow) -> Result<IdRow<DbItemLabelId>> {
        let sql = /* language=sql */ r#"
            INSERT INTO "database_labels" ("uri", "scope", "key", "value", "created_at")
            VALUES
            (?, ?, ?, ?, ?)
            RETURNING "id"
        "#;

        query_as(sql)
            .bind(create.uri)
            .bind(create.scope)
            .bind(create.key)
            .bind(create.value)
            .bind(create.created_at)
            .fetch_one(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn get_by_id(&mut self, id: DbItemLabelId) -> Result<DbItemLabelRow> {
        let sql = /* language=sql */ r#"
            SELECT "id", "uri", "scope", "key", "value", "created_at"
            FROM "database_labels"
            WHERE "id" = ?
        "#;

        query_as(sql)
            .bind(id)
            .fetch_one(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn get_all(&mut self) -> Result<Vec<DbItemLabelRow>> {
        let sql = /* language=sql */ r#"
            SELECT "id", "uri", "scope", "key", "value", "created_at"
            FROM "database_labels"
        "#;

        query_as(sql)
            .fetch_all(&mut *self.inner)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_by_id(&mut self, id: DbItemLabelId) -> Result<()> {
        let sql = /* language=sql */ r#"
            DELETE FROM "database_labels"
            WHERE "id" = ?
        "#;

        query(sql)
            .bind(id)
            .execute(&mut *self.inner)
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}

// </editor-fold>

macro_rules! impl_repository {
    ($name:ident) => {
        impl $name {
            pub fn playback_history_event(&mut self) -> PlaybackHistoryEventRepository {
                PlaybackHistoryEventRepository { inner: &mut self.inner }
            }

            pub fn playback_history_metadata(&mut self) -> PlaybackHistoryMetadataRepository {
                PlaybackHistoryMetadataRepository { inner: &mut self.inner }
            }

            pub fn db_item_label(&mut self) -> DbItemLabelRepository {
                DbItemLabelRepository { inner: &mut self.inner }
            }
        }
    }
}

pub struct Connection {
    inner: PoolConnection<Sqlite>,
}

pub struct Transaction {
    inner: sqlx::Transaction<'static, Sqlite>,
}

impl_repository!(Connection);
impl_repository!(Transaction);

impl Transaction {
    pub async fn commit(self) -> Result<()> {
        self.inner.commit().await
            .map_err(Into::into)
    }
}

#[derive(Clone)]
pub struct Pool {
    inner: SqlitePool,
}

impl Pool {
    pub fn new(pool: SqlitePool) -> Self {
        Pool { inner: pool }
    }
}

impl Pool {
    pub async fn acquire(&self) -> Result<Connection> {
        let c = self.inner.acquire().await?;

        Ok(Connection { inner: c })
    }

    pub async fn begin(&self) -> Result<Transaction> {
        let tx = self.inner.begin().await?;

        Ok(Transaction { inner: tx })
    }
}
