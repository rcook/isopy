use anyhow::{bail, Result};
use isopy_api::{Accept, Context, PackageManager, Url};
use serde_json::Value;
use std::fs::read_to_string;
use std::sync::LazyLock;

const INDEX_URL: LazyLock<Url> = LazyLock::new(|| {
    "https://api.github.com/repos/indygreg/python-build-standalone/releases"
        .parse()
        .expect("Invalid index URL")
});

pub struct PythonPackageManager {
    name: String,
}

impl PythonPackageManager {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

macro_rules! g {
    ($e : expr) => {
        match $e {
            Some(value) => value,
            None => bail!("Invalid index"),
        }
    };
}

impl PackageManager for PythonPackageManager {
    fn name(&self) -> &str {
        &self.name
    }

    fn test(&self, ctx: &dyn Context) -> Result<()> {
        fn download_json(ctx: &dyn Context, url: &Url) -> Result<Value> {
            let path = ctx.download(url, Some(Accept::ApplicationJson))?;
            let s = read_to_string(path)?;
            let value = serde_json::from_str(&s)?;
            Ok(value)
        }

        let index = download_json(ctx, &INDEX_URL)?;
        let items = g!(index.as_array());
        println!("{}", items.len());
        Ok(())
    }
}
