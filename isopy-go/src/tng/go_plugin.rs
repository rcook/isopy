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
use async_trait::async_trait;
use isopy_lib::tng::{
    PackageManager, PackageManagerContext, PackageManagerOps, Plugin, PluginOps, Version,
};
use log::warn;
use std::path::Path;

pub(crate) struct GoPlugin;

impl GoPlugin {
    pub(crate) fn new() -> Plugin {
        Plugin::new(Box::new(Self))
    }
}

impl PluginOps for GoPlugin {
    fn parse_version(&self, _s: &str) -> Result<Version> {
        todo!()
    }

    fn new_package_manager(&self, _ctx: PackageManagerContext) -> PackageManager {
        struct DummyManager;

        #[async_trait]
        impl PackageManagerOps for DummyManager {
            async fn update_index(&self) -> Result<()> {
                warn!("GoPlugin: not implemented!");
                Ok(())
            }

            async fn list_categories(&self) -> Result<()> {
                todo!()
            }

            async fn list_packages(&self) -> Result<()> {
                warn!("GoPlugin: not implemented!");
                Ok(())
            }

            async fn download_package(&self, _version: &Version) -> Result<()> {
                todo!()
            }

            async fn install_package(&self, _version: &Version, _dir: &Path) -> Result<()> {
                todo!()
            }
        }

        PackageManager::new(Box::new(DummyManager))
    }
}
