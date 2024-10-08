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
use anyhow::Result;
use reqwest::Client;
use url::Url;

pub(crate) struct Adoptium {
    server_url: Url,
    client: Client,
}

impl Adoptium {
    #[allow(unused)]
    #[must_use]
    pub(crate) fn new(server_url: &Url) -> Self {
        Self {
            server_url: server_url.clone(),
            client: Client::new(),
        }
    }

    #[allow(unused)]
    pub(crate) async fn demo(&self) -> Result<()> {
        let request_builder = self
            .client
            .get(self.server_url.join("/v3/info/release_versions")?);
        let response = request_builder.send().await?;
        todo!("response={response:?}");
    }
}
