use super::{Repository, Response, Stream};
use crate::object_model::{Asset, LastModified};
use crate::result::{other, Result};
use crate::util::{to_last_modified, to_system_time, ContentLength};
use async_trait::async_trait;
use bytes::Bytes;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::PathBuf;

pub struct LocalRepository {
    dir: PathBuf,
}

impl LocalRepository {
    pub fn new<P>(dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { dir: dir.into() }
    }
}

#[async_trait]
impl Repository for LocalRepository {
    async fn get_latest_index(
        &self,
        last_modified: &Option<LastModified>,
    ) -> Result<Option<Box<dyn Response>>> {
        let index_json_path = self.dir.join("assets").join("index.json");

        let m = metadata(&index_json_path)?;

        let modified = m.modified()?;

        if let Some(l) = last_modified {
            if modified <= to_system_time(l)? {
                return Ok(None);
            }
        }

        let new_last_modified = to_last_modified(&modified)?;
        let content_length = m.len();
        Ok(Some(Box::new(LocalResponse::new(
            Some(new_last_modified),
            content_length,
            index_json_path,
        ))))
    }

    async fn get_asset(&self, asset: &Asset) -> Result<Box<dyn Response>> {
        let asset_path = self.dir.join("assets").join(&asset.name);
        let m = metadata(&asset_path)?;
        let last_modified = to_last_modified(&m.modified()?)?;
        let content_length = m.len();
        Ok(Box::new(LocalResponse::new(
            Some(last_modified),
            content_length,
            asset_path,
        )))
    }
}

struct LocalResponse {
    last_modified: Option<LastModified>,
    content_length: ContentLength,
    path: PathBuf,
}

impl LocalResponse {
    fn new<P>(last_modified: Option<LastModified>, content_length: ContentLength, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            last_modified,
            content_length,
            path: path.into(),
        }
    }
}

impl Response for LocalResponse {
    fn last_modified(&self) -> &Option<LastModified> {
        &self.last_modified
    }

    fn content_length(&self) -> Option<ContentLength> {
        Some(self.content_length)
    }

    fn bytes_stream(&mut self) -> Result<Box<dyn Stream>> {
        let file = File::open(&self.path)?;
        Ok(Box::new(LocalStream::new(file)))
    }
}

struct LocalStream {
    file: File,
}

impl LocalStream {
    fn new(file: File) -> Self {
        Self { file }
    }
}

#[async_trait]
impl Stream for LocalStream {
    async fn next(&mut self) -> Option<Result<Bytes>> {
        // https://stackoverflow.com/questions/55555538/what-is-the-correct-way-to-read-a-binary-file-in-chunks-of-a-fixed-size-and-stor
        let chunk_size = 0x4000;
        let mut chunk = Vec::with_capacity(chunk_size);
        match self
            .file
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)
        {
            Err(e) => Some(Err(other(Box::new(e)))),
            Ok(count) => {
                if count == 0 {
                    None
                } else {
                    Some(Ok(Bytes::from(chunk)))
                }
            }
        }
    }
}
