// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::download_package_options::DownloadPackageOptions;
use crate::error::InstallPackageError;
use crate::install_package_options::InstallPackageOptions;
use crate::macros::dyn_trait_struct;
use crate::package::Package;
use crate::package_summary::PackageSummary;
use crate::tags::Tags;
use crate::version::Version;
use crate::PackageFilter;
use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::result::Result as StdResult;

pub type OptionalTags = Option<Vec<String>>;

#[async_trait]
pub trait PackageManagerOps: Send + Sync {
    async fn update_index(&self) -> Result<()>;
    async fn list_tags(&self) -> Result<Tags>;
    async fn list_packages(
        &self,
        filter: PackageFilter,
        tags: &OptionalTags,
    ) -> Result<Vec<PackageSummary>>;
    async fn download_package(
        &self,
        version: &Version,
        tags: &OptionalTags,
        options: &DownloadPackageOptions,
    ) -> Result<()>;
    async fn install_package(
        &self,
        version: &Version,
        tags: &OptionalTags,
        dir: &Path,
        options: &InstallPackageOptions,
    ) -> StdResult<Package, InstallPackageError>;
    async fn on_before_install(&self, _output_dir: &Path, _bin_subdir: &Path) -> Result<()>;
    async fn on_after_install(&self, output_dir: &Path, bin_subdir: &Path) -> Result<()>;
}
dyn_trait_struct!(PackageManager, PackageManagerOps);
