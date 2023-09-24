use std::cmp::Ordering;

use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;

use crate::mpd::action::CoverArtKind;
use crate::mpd::action::QueueSource;
use crate::mpd::client::Binary;
use crate::mpd::client::escape;
use crate::mpd::client::Client;
use crate::mpd::data::DbCount;
use crate::mpd::data::DbItem;
use crate::mpd::data::OneshotState;
use crate::mpd::data::Playlist;
use crate::mpd::data::QueueItem;
use crate::mpd::data::Status;
use crate::mpd::data::to_state_string;
use crate::mpd::Error;
use crate::mpd::result::Result;

pub struct Service<'a> {
    client: &'a mut Client,
}

impl<'a> Service<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Service { client }
    }
}

pub struct DbService<'a> {
    inner: &'a mut Service<'a>,
}

fn file_filter(query: &str) -> String {
    let query = escape(query);

    format!(r#"(file =~ "{query}")"#)
}

fn base_filter(uri: &str) -> String {
    let uri = escape(uri);

    format!(r#"(base "{uri}")"#)
}

// Since MPD does not allow us to search for directories we go over
// every path segment in a URI that definitely contains a match somewhere
// and construct pseudo-results for matching directories. We also include the
// item itself if its basename matches the query.
//
// For example consider:
//
// query: test
// item.uri: alfa/test/beta/test.flac
//
// We'll get the following result:
// [DbItem::Directory { uri: "alfa/test" }, DbItem::File { uri: "alfa/test/beta/test.flac" }]
fn uri_matches(item: DbItem, query: &str) -> Result<Vec<DbItem>> {
    const PATH_SEPARATOR: &str = "/";

    let DbItem::File { uri, .. } = &item else {
        return Err(Error::Internal("uri matches should only be extracted from files".to_owned()))
    };

    let path_segments = uri.split(PATH_SEPARATOR)
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut items = path_segments.iter()
        // Skip the basename for now, we'll deal with later because ownership.
        .take(path_segments.len() - 1)
        .enumerate()
        .filter_map(|(i, segment)| {
            if !segment.to_lowercase().contains(query) {
                return None;
            }

            let mut uri = path_segments.iter()
                .take(i + 1)
                .flat_map(|s| [s, PATH_SEPARATOR])
                .collect::<String>();

            uri.pop();

            Some(DbItem::Directory { uri })
        })
        .collect::<Vec<_>>();

    // Handle the basename match.
    if path_segments.last().map(|it| it.to_lowercase().contains(query)) == Some(true) {
        items.push(item);
    }

    Ok(items)
}

impl<'a> DbService<'a> {
    pub async fn get(&mut self, uri: String) -> Result<Vec<DbItem>> {
        let result = self.inner.client.lsinfo(uri).await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, >>()?;

        Ok(result)
    }

    pub async fn count(&mut self, uri: String) -> Result<DbCount> {
        let result = self.inner.client.count(base_filter(&uri)).await?;

        Ok(result.into())
    }

    pub async fn search(&mut self, query: String) -> Result<Vec<DbItem>> {
        let query = query.to_lowercase();

        let mut items = self.inner.client.search(file_filter(&query)).await?
            .into_iter()
            .map(|it| uri_matches(it.try_into()?, &query))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        items.sort_by(|a, b| {
            match (a, b) {
                | (DbItem::File { uri: a, .. }, DbItem::File { uri: b, .. })
                | (DbItem::Directory { uri: a, .. }, DbItem::Directory { uri: b, .. })
                | (DbItem::Playlist { uri: a, .. }, DbItem::Playlist { uri: b, .. }) => a.cmp(b),
                (DbItem::Playlist { .. }, _) => Ordering::Less,
                (_, DbItem::Playlist { .. }) => Ordering::Greater,
                (DbItem::File { .. }, DbItem::Directory { .. }) => Ordering::Greater,
                (DbItem::Directory { .. }, DbItem::File { .. }) => Ordering::Less,
            }
        });

        items.dedup_by(|a, b| a.uri() == b.uri());

        Ok(items)
    }

    pub async fn update(&mut self, uri: Option<String>) -> Result<()> {
        self.inner.client.update(uri).await?;

        Ok(())
    }

    pub async fn cover_art(&mut self, uri: String, kind: CoverArtKind) -> Result<Bytes> {
        let mut result = BytesMut::new();

        let mut size = usize::MAX;
        let mut offset = 0;

        while offset < size {
            let Binary(info, data) = match kind {
                CoverArtKind::File => {
                    self.inner.client.albumart(uri.clone(), offset).await?
                },
                CoverArtKind::Embedded => {
                    self.inner.client.readpicture(uri.clone(), offset).await?
                        .ok_or_else(|| Error::NotFound(format!("file at uri '{uri}' exists, but has no embedded cover art")))?
                },
            };

            size = info.size;
            offset += info.binary;

            result.put_slice(&data);
        }

        Ok(result.freeze())
    }
}

pub struct QueueService<'a> {
    inner: &'a mut Service<'a>,
}

impl<'a> QueueService<'a> {
    pub async fn add(&mut self, source: QueueSource) -> Result<()> {
        match source {
            QueueSource::File { uri } => self.inner.client.add(uri).await?,
            QueueSource::Playlist { name } => self.inner.client.load(name).await?,
        };

        Ok(())
    }

    pub async fn replace(&mut self, source: QueueSource) -> Result<()> {
        self.inner.client
            .command_list(|builder| {
                let builder = builder.clear();

                let builder = match source {
                    QueueSource::File { uri } => builder.add(uri),
                    QueueSource::Playlist { name } => builder.load(name),
                };

                builder.playid(None)
            })
            .await?;

        Ok(())
    }

    pub async fn get(&mut self) -> Result<Vec<QueueItem>> {
        let result = self.inner.client.playlistinfo().await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_>>()?;

        Ok(result)
    }

    pub async fn clear(&mut self) -> Result<()> {
        self.inner.client.clear().await?;

        Ok(())
    }

    pub async fn remove(&mut self, id: i64) -> Result<()> {
        self.inner.client.deleteid(id).await?;

        Ok(())
    }

    pub async fn next(&mut self) -> Result<()> {
        self.inner.client.next().await?;

        Ok(())
    }

    pub async fn prev(&mut self) -> Result<()> {
        self.inner.client.previous().await?;

        Ok(())
    }

    pub async fn repeat(&mut self, state: bool) -> Result<()> {
        let state = to_state_string(state);

        self.inner.client.repeat(state).await?;

        Ok(())
    }

    pub async fn consume(&mut self, state: OneshotState) -> Result<()> {
        self.inner.client.consume(state.to_state_string()).await?;

        Ok(())
    }

    pub async fn random(&mut self, state: bool) -> Result<()> {
        let state = to_state_string(state);

        self.inner.client.random(state).await?;

        Ok(())
    }

    pub async fn single(&mut self, state: OneshotState) -> Result<()> {
        self.inner.client.single(state.to_state_string()).await?;

        Ok(())
    }
}

pub struct PlaylistService<'a> {
    inner: &'a mut Service<'a>,
}

impl<'a> PlaylistService<'a> {
    pub async fn get(&mut self, name: String) -> Result<Vec<DbItem>> {
        let result = self.inner.client.listplaylistinfo(name).await?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_>>()?;

        Ok(result)
    }

    pub async fn list(&mut self) -> Result<Vec<Playlist>> {
        let result = self.inner.client.listplaylists().await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    pub async fn delete(&mut self, name: String) -> Result<()> {
        self.inner.client.rm(name).await?;

        Ok(())
    }

    pub async fn delete_songs(&mut self, name: String, positions: Vec<usize>) -> Result<()> {
        self.inner.client
            .command_list(|builder| {
                positions.iter().fold(builder, |it, &position| {
                    it.playlistdelete(name.clone(), position)
                })
            })
            .await?;

        Ok(())
    }
}

pub struct PlaybackService<'a> {
    inner: &'a mut Service<'a>,
}

impl<'a> PlaybackService<'a> {
    pub async fn play(&mut self, id: Option<i64>) -> Result<()> {
        self.inner.client.playid(id).await?;

        Ok(())
    }

    pub async fn toggle(&mut self) -> Result<()> {
        self.inner.client.pause().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.inner.client.stop().await?;

        Ok(())
    }

    pub async fn seek(&mut self, time: f64) -> Result<()> {
        self.inner.client.seekcur(time.to_string()).await?;

        Ok(())
    }
}

pub struct StatusService<'a> {
    inner: &'a mut Service<'a>,
}

impl<'a> StatusService<'a> {
    pub async fn get(&mut self) -> Result<Status> {
        let result = self.inner.client.status().await?;

        Ok(result.try_into()?)
    }
}

pub struct VolumeService<'a> {
    inner: &'a mut Service<'a>,
}

impl<'a> VolumeService<'a> {
    pub async fn set(&mut self, value: u8) -> Result<()> {
        self.inner.client.setvol(value).await?;

        Ok(())
    }
}

impl<'a> Service<'a> {
    pub fn db(&'a mut self) -> DbService {
        DbService { inner: self }
    }

    pub fn queue(&'a mut self) -> QueueService {
        QueueService { inner: self }
    }

    pub fn playlists(&'a mut self) -> PlaylistService {
        PlaylistService { inner: self }
    }

    pub fn playback(&'a mut self) -> PlaybackService {
        PlaybackService { inner: self }
    }

    pub fn status(&'a mut self) -> StatusService {
        StatusService { inner: self }
    }

    pub fn volume(&'a mut self) -> VolumeService {
        VolumeService { inner: self }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::mpd::data::DbTags;

    use super::*;

    #[test]
    fn should_get_uri_matches() {
        let file = DbItem::File {
            uri: "alfa/test/beta/test.flac".to_owned(),
            duration: None,
            tags: DbTags {
                titles: vec!["Test".to_owned()],
                artists: vec!["Test".to_owned()],
                albums: vec![],
            },
            format: None,
        };

        let actual = uri_matches(file.clone(), "tes").unwrap();

        let expected = vec![
            DbItem::Directory { uri: "alfa/test".to_owned() },
            file,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_get_no_uri_matches() {
        let file = DbItem::File {
            uri: "alfa/test/beta/test.flac".to_owned(),
            duration: None,
            tags: DbTags {
                titles: vec!["tseT".to_owned()],
                artists: vec!["tseT".to_owned()],
                albums: vec![],
            },
            format: None,
        };

        let actual = uri_matches(file, "?????").unwrap();

        assert_eq!(actual, vec![]);
    }

    #[test]
    #[should_panic]
    fn should_panic_uri_matches() {
        let directory = DbItem::Directory { uri: "dir".to_owned() };

        uri_matches(directory, "dir").unwrap();
    }
}
