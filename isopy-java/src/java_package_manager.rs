use anyhow::Result;
use isopy_api::{Context, PackageManager};

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

    fn test(&self, _ctx: &dyn Context) -> Result<()> {
        Ok(())
    }
}
