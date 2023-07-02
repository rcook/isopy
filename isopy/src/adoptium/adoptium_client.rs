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
use super::query::Query;
use crate::api::adoptium::{List, Release, VersionData, Versions};
use crate::download::download_stream;
use crate::link_header::LinkHeader;
use crate::openjdk::MavenVersionLimit::{Closed, Open};
use crate::openjdk::MavenVersionRange::{self, Range};
use crate::openjdk::MavenVersionValue;
use crate::repository::{ReqwestResponse, Response};
use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use std::path::Path;

lazy_static! {
    static ref ALL_VERSIONS: MavenVersionRange = Range(
        Open(None),
        Closed(Some(MavenVersionValue::new(1_000_000, Some(0)))),
    );
}

pub fn all_versions() -> &'static MavenVersionRange {
    &ALL_VERSIONS
}

pub struct AdoptiumClient {
    server_url: Url,
    client: Client,
}

impl AdoptiumClient {
    pub fn new(server_url: &Url) -> Self {
        let client = Client::new();
        Self {
            server_url: server_url.clone(),
            client,
        }
    }

    #[allow(unused)]
    pub async fn get_versions(&self, query: &Query) -> Result<Vec<VersionData>> {
        self.get_list::<Versions>(
            query.apply(
                self.client
                    .get(self.server_url.join("/v3/info/release_versions")?),
            ),
        )
        .await
    }

    pub async fn get_releases(
        &self,
        version_range: &MavenVersionRange,
        query: &Query,
    ) -> Result<Vec<Release>> {
        self.get_list::<Vec<Release>>(
            query.apply(
                self.client
                    .get(Self::make_version_url(&self.server_url, version_range)?),
            ),
        )
        .await
    }

    pub async fn download_asset(&self, url: &Url, output_path: &Path) -> Result<()> {
        let request = self.client.get(url.clone());
        let response = request.send().await?;
        let mut wrapped: Box<dyn Response> = Box::new(ReqwestResponse::new(None, response));
        download_stream("JDK asset", &mut wrapped, output_path).await
    }

    async fn get_list<R>(&self, mut request_builder: RequestBuilder) -> Result<Vec<R::Item>>
    where
        R: DeserializeOwned + List,
    {
        let mut items = Vec::new();
        loop {
            let response = request_builder.send().await?.error_for_status()?;
            let next_url = LinkHeader::from_response(&response)?.and_then(|x| x.next);
            items.extend(response.json::<R>().await?.items());

            let Some(n) = next_url else {
                return Ok(items)
            };

            request_builder = self.client.get(n);
        }
    }

    fn make_version_url(server_url: &Url, version_range: &MavenVersionRange) -> Result<Url> {
        let mut url = server_url.join("/v3/assets/version")?;
        url.set_path(&format!(
            "{}/{}",
            url.path(),
            version_range.to_path_segment()
        ));
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::AdoptiumClient;
    use crate::openjdk::MavenVersionLimit::{Closed, Open};
    use crate::openjdk::MavenVersionRange::{self, Range, Value};
    use crate::openjdk::MavenVersionValue;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(
        "http://host/v3/assets/version/10",
        Value(MavenVersionValue::new(10, None))
    )]
    #[case(
        "http://host/v3/assets/version/%28%2C1000000.0%5D",
        Range(Open(None), Closed(Some(MavenVersionValue::new(1_000_000, Some(0)))),)
    )]
    fn basics(#[case] expected_str: &str, #[case] version_range: MavenVersionRange) -> Result<()> {
        assert_eq!(
            expected_str,
            AdoptiumClient::make_version_url(&"http://host".parse()?, &version_range)?.as_str()
        );
        Ok(())
    }
}
