use crate::tng::app::App;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use isopy_lib::tng::{Accept, Context, Url};
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use serde_json::Value;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use tokio::fs::read_to_string;

pub(crate) struct AppContext<'a> {
    app: &'a App,
    name: String,
}

impl<'a> AppContext<'a> {
    pub(crate) fn new<S>(app: &'a App, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            app,
            name: name.into(),
        }
    }
}

#[async_trait]
impl<'a> Context for AppContext<'a> {
    async fn download(&self, url: &Url, accept: Option<Accept>) -> Result<PathBuf> {
        let p = url.make_path(self.app.cache_dir().join(&self.name))?;
        if p.is_file() {
            println!("Returning {url} from cache");
            return Ok(p);
        }

        let dir = p
            .parent()
            .ok_or_else(|| anyhow!("Cannot get parent directory from path {}", p.display()))?;
        create_dir_all(dir)?;

        println!("Downloading {url}");

        let mut request = Client::new()
            .get(ReqwestUrl::parse(url.as_str())?)
            .header(USER_AGENT, "isopy-tng");

        if let Some(accept) = accept {
            request = request.header(ACCEPT, accept.as_str())
        };

        let response = request.send().await?;
        response.error_for_status_ref()?;

        let data = response.bytes().await?;
        write(&p, data)?;
        println!("Downloaded {url}");

        return Ok(p);
    }

    async fn download_json(&self, url: &Url) -> Result<Value> {
        let path = self.download(url, Some(Accept::ApplicationJson)).await?;
        let s = read_to_string(path).await?;
        let value = serde_json::from_str(&s)?;
        Ok(value)
    }
}
