use crate::context::Context;
use crate::package_version::PackageVersion;
use anyhow::Result;

pub trait PackageManager {
    fn name(&self) -> &str;
    fn download_package(&self, ctx: &dyn Context, version: &PackageVersion) -> Result<()>;
}
