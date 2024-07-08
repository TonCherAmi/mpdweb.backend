use std::collections::HashMap;

use crate::persist;

#[derive(Clone)]
pub struct Handle {
    inner: persist::Handle,
}

impl Handle {
    pub fn new(persistence_handle: persist::Handle) -> Self {
        Handle { inner: persistence_handle }
    }
}

pub struct DbItemLabel {
    pub id: i64,
    pub scope: String,
    pub key: String,
    pub value: String,
    pub created_at: time::OffsetDateTime,
}

impl From<persist::DbItemLabel> for DbItemLabel {
    fn from(persist::DbItemLabel {
        id,
        uri: _uri,
        scope,
        key,
        value,
        created_at
    }: persist::DbItemLabel) -> Self {
        DbItemLabel {
            id,
            scope,
            key,
            value,
            created_at,
        }
    }
}

pub struct CreateDbItemLabel {
    pub uri: String,
    pub scope: String,
    pub key: String,
    pub value: String,
}

impl From<CreateDbItemLabel> for persist::CreateDbItemLabel {
    fn from(CreateDbItemLabel {
        uri,
        scope,
        key,
        value
    }: CreateDbItemLabel) -> Self {
        persist::CreateDbItemLabel {
            uri,
            scope,
            key,
            value,
        }
    }
}

impl Handle {
    pub async fn create(&self, label: CreateDbItemLabel) -> Result<DbItemLabel, String> {
        let label = self.inner.db_item_label().create(label.into()).await?;

        Ok(label.into())
    }

    pub async fn delete(&self, id: i64) -> Result<(), String> {
        self.inner.db_item_label().delete_by_id(id).await?;

        Ok(())
    }

    pub async fn get_all_grouped_by_uri(&self) -> Result<HashMap<String, Vec<DbItemLabel>>, String> {
        let labels = self.inner.db_item_label().get_all().await?;

        let mut map: HashMap<String, Vec<DbItemLabel>> = HashMap::new();

        for label in labels {
            map.entry(label.uri.clone())
                .or_default()
                .push(label.into());
        }

        Ok(map)
    }
}
