use super::{Repository, Response, Stream};
use crate::result::{Error, Result};
use crate::util::ContentLength;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::Stream as FuturesStream;
use futures_util::StreamExt;
use reqwest::{Client, IntoUrl, Response as ReqwestResponse, Url};
use std::pin::Pin;

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
            url: url.into_url()?,
            client: Client::new(),
        })
    }
}

#[async_trait]
impl Repository for GitHubRepository {
    async fn get_index(&self) -> Result<Box<dyn Response>> {
        let index_json_url = self.url.join("/assets/index.json")?;
        Ok(Box::new(GitHubResponse::new(
            self.client.get(index_json_url).send().await?,
        )))
    }
}

struct GitHubResponse {
    response: Option<ReqwestResponse>,
    content_length: Option<ContentLength>,
}

impl GitHubResponse {
    fn new(response: ReqwestResponse) -> Self {
        let content_length = response.content_length();
        Self {
            response: Some(response),
            content_length: content_length,
        }
    }
}

impl Response for GitHubResponse {
    fn content_length(&self) -> Option<ContentLength> {
        self.content_length
    }

    fn bytes_stream(&mut self) -> Result<Box<dyn Stream>> {
        let response = self
            .response
            .take()
            .ok_or(Error::Fatal(String::from("Response already consumed")))?;
        let stream = response.bytes_stream();
        Ok(Box::new(GitHubStream::new(Box::pin(stream))))
    }
}

struct GitHubStream {
    stream: PinnedStream,
}

impl GitHubStream {
    fn new(stream: PinnedStream) -> Self {
        Self { stream: stream }
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
