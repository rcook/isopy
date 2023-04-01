use super::{Repository, Response, ResponseInfo, Stream};
use crate::object_model::LastModified;
use crate::result::{Error, Result};
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
    ) -> Result<Option<ResponseInfo>> {
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
        let resp = Box::new(LocalResponse::new(index_json_path, content_length));
        Ok(Some((new_last_modified, Some(content_length), resp)))
    }
}

struct LocalResponse {
    path: PathBuf,
    content_length: ContentLength,
}

impl LocalResponse {
    fn new<P>(path: P, content_length: ContentLength) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into(),
            content_length: content_length,
        }
    }
}

impl Response for LocalResponse {
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
        Self { file: file }
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
            Err(e) => Some(Err(Error::Other(Box::new(e)))),
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
