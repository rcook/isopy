use crate::app::App;
use anyhow::{anyhow, Result};
use isopy_api::{Accept, Context, Url};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::Url as ReqwestUrl;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

pub struct AppContext<'a> {
    app: &'a App,
    name: String,
}

impl<'a> AppContext<'a> {
    pub fn new<S>(app: &'a App, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            app,
            name: name.into(),
        }
    }
}

impl<'a> Context for AppContext<'a> {
    fn download(&self, url: &Url, accept: Option<Accept>) -> Result<PathBuf> {
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

        let response = request.send()?;
        response.error_for_status_ref()?;

        let data = response.bytes()?;
        write(&p, data)?;
        println!("Downloaded {url}");

        return Ok(p);
    }
}
