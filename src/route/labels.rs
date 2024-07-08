use std::collections::HashMap;

use axum::Extension;
use axum::extract::Path;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

use crate::convert::MapInto;
use crate::labels;
use crate::labels::CreateDbItemLabel;
use crate::route::result::Result;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DbItemLabel {
    id: String,
    scope: String,
    key: String,
    value: String,
    #[serde(with = "time::serde::iso8601")]
    created_at: OffsetDateTime,
}

impl From<labels::DbItemLabel> for DbItemLabel {
    fn from(labels::DbItemLabel {
        id,
        scope,
        key,
        value,
        created_at
    }: labels::DbItemLabel) -> Self {
        DbItemLabel {
            id: id.to_string(),
            scope,
            key,
            value,
            created_at,
        }
    }
}

type LabelsByUri = HashMap<String, Vec<DbItemLabel>>;

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn labels(
    Extension(handle): Extension<labels::Handle>,
) -> Result<Json<LabelsByUri>> {
    let result: LabelsByUri = handle.get_all_grouped_by_uri()
        .await?
        .drain()
        .map(|(uri, xs)| (uri, xs.map_into()))
        .collect();

    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct CreateDbItemLabelBody {
    uri: String,
    scope: String,
    key: String,
    value: String,
}

impl From<CreateDbItemLabelBody> for CreateDbItemLabel {
    fn from(CreateDbItemLabelBody {
        uri,
        scope,
        key,
        value
    }: CreateDbItemLabelBody) -> Self {
        CreateDbItemLabel {
            uri,
            scope,
            key,
            value,
        }
    }
}

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn create(
    Extension(handle): Extension<labels::Handle>,
    Json(body): Json<CreateDbItemLabelBody>,
) -> Result<Json<DbItemLabel>> {
    let result = handle.create(body.into())
        .await?;

    Ok(Json(result.into()))
}

#[derive(Debug, Deserialize)]
pub struct DeleteDbItemLabelPathParams {
    id: i64,
}

#[tracing::instrument(ret, skip(handle), level = "debug")]
pub async fn delete(
    Path(params): Path<DeleteDbItemLabelPathParams>,
    Extension(handle): Extension<labels::Handle>,
) -> Result<()> {
    handle.delete(params.id).await?;

    Ok(())
}