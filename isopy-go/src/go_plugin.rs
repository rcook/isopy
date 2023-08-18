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
use crate::constants::{ASSETS_DIR, INDEX_FILE_NAME};
use crate::download::download_index;
use crate::filter::Filter;
use crate::go_version::GoVersion;
use crate::serialization::IndexRec;
use async_trait::async_trait;
use isopy_lib::{other_error, Descriptor, IsopyLibResult, Package, Plugin};
use joatmon::read_yaml_file;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct GoPlugin {
    _offline: bool,
    dir: PathBuf,
    assets_dir: PathBuf,
}

impl GoPlugin {
    pub fn new(offline: bool, dir: &Path) -> Self {
        let assets_dir = dir.join(&*ASSETS_DIR);
        Self {
            _offline: offline,
            dir: dir.to_path_buf(),
            assets_dir,
        }
    }

    async fn get_available_items(&self) -> IsopyLibResult<Vec<(Package, GoVersion)>> {
        let index_path = self.dir.join(&*INDEX_FILE_NAME);

        download_index(&Filter::default(), &index_path)
            .await
            .map_err(other_error)?;

        let index_rec = read_yaml_file::<IndexRec>(&index_path).map_err(other_error)?;

        let mut items = Vec::new();
        for version_rec in index_rec.versions {
            let asset_path = self.assets_dir.join(version_rec.file_name);
            let version = version_rec
                .version
                .parse::<GoVersion>()
                .map_err(other_error)?;

            let descriptor: Arc<Box<dyn Descriptor>> = Arc::new(Box::new(version.clone()));

            let package = Package {
                asset_path,
                descriptor,
            };

            items.push((package, version));
        }

        items.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(items)
    }
}

#[async_trait]
impl Plugin for GoPlugin {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let items = self.get_available_items().await?;
        Ok(items.into_iter().map(|p| p.0).collect::<Vec<_>>())
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        let mut items: Vec<(Package, GoVersion)> = Vec::new();

        if self.assets_dir.exists() {
            let available_items = self.get_available_items().await?;
            for item in available_items {
                println!("{}", item.1.raw);
            }
            todo!();
        }

        items.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(items.into_iter().map(|p| p.0).collect::<Vec<_>>())
    }

    async fn download_package(&self, _descriptor: &dyn Descriptor) -> IsopyLibResult<Package> {
        todo!();
    }
}
