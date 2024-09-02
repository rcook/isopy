use crate::tng::app_context::AppContext;
use anyhow::Result;
use isopy_lib::tng::{PackageManager, PackageVersion};
use std::path::Path;

pub(crate) struct AppPackageManager {
    ctx: AppContext,
    inner: PackageManager,
}

impl AppPackageManager {
    pub(crate) fn new(ctx: AppContext, inner: PackageManager) -> Self {
        Self { ctx, inner }
    }

    #[allow(unused)]
    pub(crate) async fn list_categories(&self) -> Result<()> {
        self.inner.list_categories(&self.ctx).await?;
        Ok(())
    }

    #[allow(unused)]
    pub(crate) async fn list_packages(&self) -> Result<()> {
        self.inner.list_packages(&self.ctx).await?;
        Ok(())
    }

    #[allow(unused)]
    pub(crate) async fn download_package(&self, version: &PackageVersion) -> Result<()> {
        self.inner.download_package(&self.ctx, version).await?;
        Ok(())
    }

    #[allow(unused)]
    pub(crate) async fn install_package(&self, version: &PackageVersion, dir: &Path) -> Result<()> {
        self.inner.install_package(&self.ctx, version, dir).await?;
        Ok(())
    }
}
