use anyhow::Result;
use isopy_api::{Context, Url};
use std::{path::PathBuf, rc::Rc};

use crate::app::App;

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
            return Ok(p);
        }

        todo!()
    }
}
