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
use crate::accept::Accept;
use crate::checksum::Checksum;
use crate::macros::dyn_trait_struct;
use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use reqwest::Response;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Builder, Default)]
#[builder(default)]
pub struct DownloadAssetOptions {
    pub show_progress: bool,
    pub update: bool,
    pub accept: Option<Accept>,
    pub checksum: Option<Checksum>,
    pub query: Vec<(String, String)>,
}

#[macro_export]
macro_rules! query {
    ($x: expr) => {
        $x.into_iter()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect()
    };
}

impl DownloadAssetOptionsBuilder {
    #[must_use]
    pub fn json() -> Self {
        let mut me = Self::default();
        me.accept(Some(Accept::ApplicationJson));
        me
    }
}

pub struct DownloadAssetResponse {
    pub path: PathBuf,
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct DownloadPaginatedAssetOptions {
    pub show_progress: bool,
    pub update: bool,
    pub accept: Option<Accept>,
    pub check: Option<fn(&Response) -> Result<()>>,
}

pub struct DownloadPaginatedAssetResponse {
    pub dir: PathBuf,
    pub parts: Vec<PathBuf>,
}

#[async_trait]
pub trait PackageManagerContextOps: Send + Sync {
    fn base_dir(&self) -> &Path;
    fn check_asset(&self, url: &Url) -> Result<Option<PathBuf>>;
    fn check_asset_dir(&self, url: &Url) -> Result<Option<PathBuf>>;
    fn make_asset_dir(&self, url: &Url, create_new: bool) -> Result<PathBuf>;
    async fn download_asset(
        &self,
        url: &Url,
        options: &DownloadAssetOptions,
    ) -> Result<DownloadAssetResponse>;
    async fn download_paginated_asset(
        &self,
        url: &Url,
        options: &DownloadPaginatedAssetOptions,
    ) -> Result<DownloadPaginatedAssetResponse>;
}
dyn_trait_struct!(PackageManagerContext, PackageManagerContextOps);
