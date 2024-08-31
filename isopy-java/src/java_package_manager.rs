use anyhow::Result;
use isopy_api::{Context, PackageManager, PackageVersion};

pub struct JavaPackageManager {
    name: String,
}

impl JavaPackageManager {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl PackageManager for JavaPackageManager {
    fn name(&self) -> &str {
        &self.name
    }

    fn download_package(&self, _ctx: &dyn Context, _version: &PackageVersion) -> Result<()> {
        todo!()
    }
}
