use super::{Repository, Response, ResponseInfo, Stream};
use crate::object_model::LastModified;
use crate::result::{Error, Result};
use crate::util::ContentLength;
use crate::util::ISOPY_USER_AGENT;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::Stream as FuturesStream;
use futures_util::StreamExt;
use reqwest::header::{IF_MODIFIED_SINCE, LAST_MODIFIED, USER_AGENT};
use reqwest::{Client, IntoUrl, Response as ReqwestResponse, StatusCode, Url};
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
    async fn get_latest_index(
        &self,
        last_modified: &Option<LastModified>,
    ) -> Result<Option<ResponseInfo>> {
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

        let index_request = self.client.get(self.url.clone());
        let index_response = index_request.send().await?;
        let content_length = index_response.content_length();
        let resp = Box::new(GitHubResponse::new(index_response));
        Ok(Some((new_last_modified, content_length, resp)))

        /*
        if response.status() != StatusCode::NOT_MODIFIED {
            println!("New releases are available");
            if let Some(last_modified) = response.headers().get(LAST_MODIFIED) {
                safe_write_file(
                    &index_yaml_path,
                    serde_yaml::to_string(&IndexRecord {
                        last_modified: LastModified::parse(last_modified.to_str()?),
                    })?,
                    true,
                )?;

                download_file(&client, RELEASES_URL, index_json_path, true).await?;
            }
        }
        */
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
