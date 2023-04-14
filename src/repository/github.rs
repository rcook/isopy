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
use crate::object_model::{Asset, LastModified};
use crate::util::{dir_url, ContentLength};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::Stream as FuturesStream;
use futures_util::StreamExt;
use reqwest::header::{IF_MODIFIED_SINCE, LAST_MODIFIED, USER_AGENT};
use reqwest::{Client, IntoUrl, Response as ReqwestResponse, StatusCode, Url};
use std::pin::Pin;

pub const ISOPY_USER_AGENT: &str = "isopy";

type PinnedStream = Pin<Box<dyn FuturesStream<Item = reqwest::Result<Bytes>> + Send>>;

pub struct GitHubRepository {
    url: Url,
    client: Client,
}

impl GitHubRepository {
    pub fn new<U>(url: U) -> Result<Self>
    where
        U: IntoUrl,
    {
        Ok(Self {
            url: dir_url(url)?,
            client: Client::new(),
        })
    }
}

#[async_trait]
impl Repository for GitHubRepository {
    async fn get_latest_index(
        &self,
        last_modified: &Option<LastModified>,
    ) -> Result<Option<Box<dyn Response>>> {
        let latest_url = self.url.join("latest")?;
        let mut head_request = self
            .client
            .head(latest_url.clone())
            .header(USER_AGENT, ISOPY_USER_AGENT);
        if let Some(x) = last_modified {
            head_request = head_request.header(IF_MODIFIED_SINCE, x.as_str());
        }

        let head_response = head_request.send().await?.error_for_status()?;
        if head_response.status() == StatusCode::NOT_MODIFIED {
            return Ok(None);
        }

        let new_last_modified = LastModified::parse(
            head_response
                .headers()
                .get(LAST_MODIFIED)
                .expect("Last-Modified header should be present")
                .to_str()?,
        );

        let index_request = self
            .client
            .get(self.url.clone())
            .header(USER_AGENT, ISOPY_USER_AGENT);
        let index_response = index_request.send().await?;
        Ok(Some(Box::new(GitHubResponse::new(
            Some(new_last_modified),
            index_response,
        ))))
    }

    async fn get_asset(&self, asset: &Asset) -> Result<Box<dyn Response>> {
        let request = self
            .client
            .get(asset.url.clone())
            .header(USER_AGENT, ISOPY_USER_AGENT);
        let response = request.send().await?;
        Ok(Box::new(GitHubResponse::new(None, response)))
    }
}

struct GitHubResponse {
    last_modified: Option<LastModified>,
    content_length: Option<ContentLength>,
    response: Option<ReqwestResponse>,
}

impl GitHubResponse {
    fn new(last_modified: Option<LastModified>, response: ReqwestResponse) -> Self {
        let content_length = response.content_length();
        Self {
            last_modified,
            content_length,
            response: Some(response),
        }
    }
}

impl Response for GitHubResponse {
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
            .ok_or(anyhow!("Response already consumed"))?;
        let stream = response.bytes_stream();
        Ok(Box::new(GitHubStream::new(Box::pin(stream))))
    }
}

struct GitHubStream {
    stream: PinnedStream,
}

impl GitHubStream {
    fn new(stream: PinnedStream) -> Self {
        Self { stream }
    }
}

unsafe impl Sync for GitHubStream {}

#[async_trait]
impl Stream for GitHubStream {
    async fn next(&mut self) -> Option<Result<Bytes>> {
        self.stream
            .next()
            .await
            .map(|x| x.map_err(reqwest::Error::into))
    }
}
