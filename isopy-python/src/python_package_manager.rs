use anyhow::Result;
use isopy_api::{Context, PackageManager};

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

impl PackageManager for PythonPackageManager {
    fn name(&self) -> &str {
        &self.name
    }

    fn test(&self, ctx: &dyn Context) -> Result<()> {
        let url = "https://raw.githubusercontent.com/indygreg/python-build-standalone/latest-release/latest-release.json".parse()?;
        let manifest_path = ctx.download(&url)?;
        println!("PYTHON! {}", manifest_path.display());
        Ok(())
    }
}
