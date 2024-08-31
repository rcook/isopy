use crate::app::App;
use anyhow::{anyhow, Result};
use isopy_api::{Context, Url};
use reqwest::blocking::get;
use reqwest::Url as ReqwestUrl;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use std::rc::Rc;

pub struct AppContext {
    app: Rc<App>,
    name: String,
}

impl AppContext {
    pub fn new<S>(app: Rc<App>, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            app,
            name: name.into(),
        }
    }
}

impl Context for AppContext {
    fn download(&self, url: &Url) -> Result<PathBuf> {
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
        let response = get(ReqwestUrl::parse(url.as_str())?)?;
        response.error_for_status_ref()?;
        let data = response.bytes()?;
        write(&p, data)?;
        println!("Downloaded {url}");
        return Ok(p);
    }
}
