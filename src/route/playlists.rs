use axum::Extension;
use axum::extract::Path;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::mpd;
use crate::route::db::DbItem;
use crate::route::result::Result;

#[derive(Debug, Deserialize)]
pub struct PlaylistPathParams {
    name: String,
}

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn playlist(
    Path(params): Path<PlaylistPathParams>,
    Extension(handle): Extension<mpd::Handle>,
) -> Result<Json<Vec<DbItem>>> {
    let items = handle.playlists().get(params.name).await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(Json(items))
}

#[derive(Debug, Serialize)]
pub struct Playlist {
    name: String,
    updated_at: String,
}

impl From<mpd::Playlist> for Playlist {
    fn from(mpd::Playlist { name, updated_at }: mpd::Playlist) -> Self {
        Playlist { name, updated_at }
    }
}

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn playlists(
    Extension(handle): Extension<mpd::Handle>,
) -> Result<Json<Vec<Playlist>>> {
    let items = handle.playlists().list().await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(Json(items))
}

#[derive(Debug, Deserialize)]
pub struct PlaylistDeletePathParams {
    name: String,
}

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn delete(
    Path(params): Path<PlaylistDeletePathParams>,
    Extension(handle): Extension<mpd::Handle>,
) -> Result<()> {
    handle.playlists().delete(params.name).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct PlaylistDeleteSongsPathParams {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistDeleteSongsBody {
    positions: Vec<usize>,
}

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn delete_songs(
    Path(params): Path<PlaylistDeleteSongsPathParams>,
    Extension(handle): Extension<mpd::Handle>,
    Json(body): Json<PlaylistDeleteSongsBody>,
) -> Result<()> {
    handle.playlists().delete_songs(params.name, body.positions).await?;

    Ok(())
}
