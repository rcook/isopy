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
use crate::descriptor::Descriptor;
use crate::env_info::EnvInfo;
use crate::package::Package;
use crate::result::IsopyLibResult;
use async_trait::async_trait;
use reqwest::Url;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub trait PluginFactory: Send + Sync {
    fn name(&self) -> &str;
    fn source_url(&self) -> &Url;
    fn make_plugin(&self, dir: &Path) -> Box<dyn PluginTNG>;
}

#[async_trait]
pub trait PluginTNG {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>>;
    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>>;
    async fn download_asset(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<PathBuf>;
}

#[async_trait]
pub trait Product: Send + Sync {
    async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        plugin_dir: &Path,
    ) -> IsopyLibResult<PathBuf>;

    fn project_config_file_name(&self) -> &OsStr;

    fn read_project_config_file(&self, path: &Path) -> IsopyLibResult<Box<dyn Descriptor>>;

    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>>;

    fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> IsopyLibResult<EnvInfo>;
}
