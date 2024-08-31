use crate::context::Context;
use crate::package_version::PackageVersion;
use anyhow::Result;

pub trait PackageManager {
    fn download_package(&self, ctx: &dyn Context, version: &PackageVersion) -> Result<()>;
}
