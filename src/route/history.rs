use axum::Extension;
use axum::extract::Query;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

use crate::convert::IntoOption;
use crate::convert::MapInto;
use crate::history;
use crate::route::db::DbTags;
use crate::route::result::Result;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    id: i64,
    uri: String,
    tags: DbTags,
    #[serde(with = "time::serde::iso8601")]
    recorded_at: OffsetDateTime,
}

impl From<history::HistoryEntry> for HistoryEntry {
    fn from(history::HistoryEntry { id, uri, tags, recorded_at, .. }: history::HistoryEntry) -> Self {
        HistoryEntry {
            id,
            uri,
            tags: tags.into(),
            recorded_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HistoryQueryParams {
    #[serde(with = "time::serde::iso8601")]
    from: OffsetDateTime,
    #[serde(default, with = "time::serde::iso8601::option")]
    to: Option<OffsetDateTime>,
}

pub async fn history(
    Query(params): Query<HistoryQueryParams>,
    Extension(handle): Extension<history::Handle>,
) -> Result<Json<Vec<HistoryEntry>>> {
    let result = handle.get(params.from.into_some(), params.to)
        .await?
        .map_into();

    Ok(Json(result))
}
