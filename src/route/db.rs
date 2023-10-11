use axum::Extension;
use axum::extract::Query;
use axum::http::header;
use axum::http::header::HeaderName;
use axum::Json;
use bytes::Bytes;
use hyper::StatusCode;
use serde::Deserialize;
use serde::Serialize;

use crate::mpd;
use crate::route::error::Error;
use crate::route::result::Result;
use crate::time::Duration;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct DbAudioFormat {
    bit_depth: i64,
    sampling_rate: i64,
    number_of_channels: i64,
}

impl From<mpd::DbAudioFormat> for DbAudioFormat {
    fn from(
        mpd::DbAudioFormat {
            bit_depth,
            sampling_rate,
            number_of_channels,
        }: mpd::DbAudioFormat,
    ) -> Self {
        DbAudioFormat {
            bit_depth,
            sampling_rate,
            number_of_channels,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DbTags {
    pub titles: Vec<String>,
    pub artists: Vec<String>,
    pub albums: Vec<String>,
}

impl From<mpd::DbTags> for DbTags {
    fn from(
        mpd::DbTags {
            artists,
            albums,
            titles,
        }: mpd::DbTags,
    ) -> Self {
        DbTags {
            artists,
            albums,
            titles,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DbItem {
    #[serde(rename_all = "camelCase")]
    File {
        uri: String,
        duration: Duration,
        tags: DbTags,
        format: Option<DbAudioFormat>,
        updated_at: String,
    },
    Directory {
        uri: String,
    },
    Playlist {
        uri: String,
    },
}

impl From<mpd::DbItem> for DbItem {
    fn from(item: mpd::DbItem) -> Self {
        use mpd::DbItem::*;

        match item {
            File {
                uri,
                duration,
                tags,
                format,
                updated_at,
            } => DbItem::File {
                uri,
                updated_at,
                duration: duration.into(),
                tags: tags.into(),
                format: format.map(Into::into),
            },
            Directory { uri } => DbItem::Directory { uri },
            Playlist { uri } => DbItem::Playlist { uri },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DbQueryParams {
    uri: Option<String>,
    query: Option<String>,
}

const MIN_QUERY_LEN: usize = 3;

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn database(
    Query(params): Query<DbQueryParams>,
    Extension(handle): Extension<mpd::Handle>,
) -> Result<Json<Vec<DbItem>>> {
    let items = match (params.uri, params.query) {
        (Some(uri), None) => handle.db().get(uri).await?,
        (None, Some(query)) if query.len() < MIN_QUERY_LEN => {
            return Err(Error::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Minimum query length is {MIN_QUERY_LEN}"),
            ))
        },
        (None, Some(query)) => handle.db().search(query).await?,
        (None, None) | (Some(_), Some(_)) => {
            return Err(Error::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Expected exactly one of uri or query to not be null".to_owned(),
            ))
        },
    };

    let items = items.into_iter().map(Into::into).collect();

    Ok(Json(items))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CoverArtKind {
    File,
    Embedded,
}

impl From<CoverArtKind> for mpd::CoverArtKind {
    fn from(kind: CoverArtKind) -> Self {
        match kind {
            CoverArtKind::File => mpd::CoverArtKind::File,
            CoverArtKind::Embedded => mpd::CoverArtKind::Embedded,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DbCoverQueryParams {
    uri: String,
    kind: CoverArtKind,
}

#[tracing::instrument(skip(handle), level = "debug")]
pub async fn cover(
    Query(params): Query<DbCoverQueryParams>,
    Extension(handle): Extension<mpd::Handle>,
) -> Result<([(HeaderName, &'static str); 1], Bytes)> {
    let result = handle
        .db()
        .cover_art(params.uri, params.kind.into())
        .await?;

    Ok(([(header::CONTENT_TYPE, "image/png")], result))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DbCount {
    song_count: i64,
    playtime: Duration,
}

impl From<mpd::DbCount> for DbCount {
    fn from(count: mpd::DbCount) -> Self {
        DbCount {
            song_count: count.nsongs,
            playtime: count.playtime.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DbCountQueryParams {
    uri: String,
}

pub async fn count(
    Query(params): Query<DbCountQueryParams>,
    Extension(handle): Extension<mpd::Handle>,
) -> Result<Json<DbCount>> {
    let result = handle.db().count(params.uri).await?.into();

    Ok(Json(result))
}

pub async fn recents(
    Extension(handle): Extension<mpd::Handle>,
) -> Result<Json<Vec<DbItem>>> {
    let items = handle.db().recents().await?;

    let items = items.into_iter().map(Into::into).collect();

    Ok(Json(items))
}
