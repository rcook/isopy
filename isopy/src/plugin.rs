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
use anyhow::Result;
use isopy_lib::{Descriptor, EnvInfo, Package, Product};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use url::Url;

pub struct Plugin {
    prefix: String,
    product: Box<dyn Product>,
}

impl Plugin {
    pub fn new(prefix: &str, product: Box<dyn Product>) -> Self {
        Self {
            prefix: String::from(prefix),
            product,
        }
    }

    pub fn name(&self) -> &str {
        self.product.name()
    }

    pub fn repository_url(&self) -> &Url {
        self.product.repository_url()
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub async fn get_available_packages(&self, shared_dir: &Path) -> Result<Vec<Package>> {
        Ok(self.product.get_available_packages(shared_dir).await?)
    }

    pub async fn get_downloaded_packages(&self, shared_dir: &Path) -> Result<Vec<Package>> {
        Ok(self.product.get_downloaded_packages(shared_dir).await?)
    }

    pub async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        plugin_dir: &Path,
    ) -> Result<PathBuf> {
        Ok(self.product.download_asset(descriptor, plugin_dir).await?)
    }

    pub fn parse_descriptor(&self, s: &str) -> Result<Box<dyn Descriptor>> {
        Ok(self.product.parse_descriptor(s)?)
    }

    pub fn project_config_file_name(&self) -> &OsStr {
        self.product.project_config_file_name()
    }

    pub fn read_project_config_file(&self, path: &Path) -> Result<Box<dyn Descriptor>> {
        Ok(self.product.read_project_config_file(path)?)
    }

    pub fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> Result<EnvInfo> {
        Ok(self.product.read_env_config(data_dir, properties)?)
    }
}
