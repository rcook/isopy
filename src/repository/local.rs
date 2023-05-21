// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use super::{Repository, Response, Stream};
use crate::constants::RELEASES_FILE_NAME;
use crate::download::ContentLength;
use crate::object_model::{Asset, LastModified};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bytes::Bytes;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct LocalRepository {
    dir: PathBuf,
}

impl LocalRepository {
    pub const fn new(dir: PathBuf) -> Self {
        Self { dir }
    }
}

#[async_trait]
impl Repository for LocalRepository {
    async fn get_latest_index(
        &self,
        last_modified: &Option<LastModified>,
    ) -> Result<Option<Box<dyn Response>>> {
        let index_json_path = self.dir.join("assets").join(RELEASES_FILE_NAME);

        let m = metadata(&index_json_path)?;

        let modified = m.modified()?;

        if let Some(l) = last_modified {
            if modified <= SystemTime::try_from(l)? {
                return Ok(None);
            }
        }

        let new_last_modified = LastModified::try_from(&modified)?;
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
        let last_modified = LastModified::try_from(&m.modified()?)?;
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
    const fn new(
        last_modified: Option<LastModified>,
        content_length: ContentLength,
        path: PathBuf,
    ) -> Self {
        Self {
            last_modified,
            content_length,
            path,
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
    const fn new(file: File) -> Self {
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
            Err(e) => Some(Err(anyhow!(e))),
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
