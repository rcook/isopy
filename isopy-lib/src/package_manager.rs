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
use crate::error::InstallPackageError;
use crate::macros::dyn_trait_struct;
use crate::package::Package;
use crate::package_state::PackageState;
use crate::tag_filter::TagFilter;
use crate::tags::Tags;
use crate::version::Version;
use crate::SourceFilter;
use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use std::path::Path;
use std::result::Result as StdResult;

#[derive(Builder, Default)]
#[builder(default)]
pub struct DownloadPackageOptions {
    pub show_progress: bool,
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct GetPackageStateOptions {
    pub show_progress: bool,
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct InstallPackageOptions {
    pub show_progress: bool,
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct ListPackageStatesOptions {
    pub show_progress: bool,
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct ListTagsOptions {
    pub show_progress: bool,
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct UpdateIndexOptions {
    pub show_progress: bool,
}

#[async_trait]
pub trait PackageManagerOps: Send + Sync {
    async fn update_index(&self, options: &UpdateIndexOptions) -> Result<()>;
    async fn list_tags(&self, options: &ListTagsOptions) -> Result<Tags>;
    async fn list_package_states(
        &self,
        sources: SourceFilter,
        tag_filter: &TagFilter,
        options: &ListPackageStatesOptions,
    ) -> Result<Vec<PackageState>>;
    async fn get_package_state(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &GetPackageStateOptions,
    ) -> Result<Option<PackageState>>;
    async fn download_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        options: &DownloadPackageOptions,
    ) -> Result<()>;
    async fn install_package(
        &self,
        version: &Version,
        tag_filter: &TagFilter,
        dir: &Path,
        options: &InstallPackageOptions,
    ) -> StdResult<Package, InstallPackageError>;
}
dyn_trait_struct!(PackageManager, PackageManagerOps);

#[cfg(test)]
mod tests {
    use crate::package_manager::ListPackageStatesOptionsBuilder;

    #[test]
    fn basics() {
        assert!(
            !ListPackageStatesOptionsBuilder::default()
                .build()
                .expect("Must succeed")
                .show_progress
        );
        assert!(
            ListPackageStatesOptionsBuilder::default()
                .show_progress(true)
                .build()
                .expect("Must succeed")
                .show_progress
        );
    }
}
