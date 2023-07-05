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
use crate::last_modified::LastModified;
use crate::response::{ContentLength, Response, Stream};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::Stream as FuturesStream;
use futures_util::StreamExt;
use std::pin::Pin;

type PinnedStream = Pin<Box<dyn FuturesStream<Item = reqwest::Result<Bytes>> + Send>>;

pub struct ReqwestResponse {
    last_modified: Option<LastModified>,
    content_length: Option<ContentLength>,
    response: Option<reqwest::Response>,
}

impl ReqwestResponse {
    pub fn new(last_modified: Option<LastModified>, response: reqwest::Response) -> Self {
        let content_length = response.content_length();
        Self {
            last_modified,
            content_length,
            response: Some(response),
        }
    }
}

impl Response for ReqwestResponse {
    fn last_modified(&self) -> &Option<LastModified> {
        &self.last_modified
    }

    fn content_length(&self) -> Option<ContentLength> {
        self.content_length
    }

    fn bytes_stream(&mut self) -> Result<Box<dyn Stream>> {
        let response = self
            .response
            .take()
            .ok_or_else(|| anyhow!("Response already consumed"))?;
        let stream = response.bytes_stream();
        Ok(Box::new(ReqwestStream::new(Box::pin(stream))))
    }
}

struct ReqwestStream {
    stream: PinnedStream,
}

impl ReqwestStream {
    fn new(stream: PinnedStream) -> Self {
        Self { stream }
    }
}

unsafe impl Sync for ReqwestStream {}

#[async_trait]
impl Stream for ReqwestStream {
    async fn next(&mut self) -> Option<Result<Bytes>> {
        self.stream
            .next()
            .await
            .map(|x| x.map_err(reqwest::Error::into))
    }
}
