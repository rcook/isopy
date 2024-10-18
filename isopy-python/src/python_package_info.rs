// Copyright (c) 2024 Richard Cook
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
use crate::index::Index;
use crate::python_package::PythonPackage;
use crate::python_version::PythonVersion;
use anyhow::Result;
use isopy_lib::{PackageAvailability, PackageInfo, PackageManagerContext, Version};
use std::collections::HashSet;
use std::path::PathBuf;

pub(crate) struct PythonPackageInfo {
    pub(crate) availability: PackageAvailability,
    pub(crate) details: PythonPackage,
    pub(crate) path: Option<PathBuf>,
}

impl PythonPackageInfo {
    pub(crate) async fn read(
        ctx: &PackageManagerContext,
        index: &Index,
        version: &PythonVersion,
        tags: &HashSet<&str>,
    ) -> Result<Option<Self>> {
        let mut packages = Vec::new();
        for item in index.items() {
            for package in PythonPackage::parse_multi(&item)? {
                let m = package.metadata();
                if m.has_tags(tags) && m.index_version().matches(version) {
                    let (availability, path) = match ctx.get_file(package.url()).await {
                        Ok(p) => (PackageAvailability::Local, Some(p)),
                        _ => (PackageAvailability::Remote, None),
                    };
                    packages.push(Self {
                        availability,
                        details: package,
                        path,
                    });
                }
            }
        }

        packages.sort_by_cached_key(|p| p.details.metadata().index_version().clone());
        packages.reverse();
        Ok(packages.into_iter().next())
    }

    pub(crate) async fn read_multi(
        ctx: &PackageManagerContext,
        index: &Index,
    ) -> Result<Vec<Self>> {
        let mut packages = Vec::new();
        for item in index.items() {
            for package in PythonPackage::parse_multi(&item)? {
                let (availability, path) = match ctx.get_file(package.url()).await {
                    Ok(p) => (PackageAvailability::Local, Some(p)),
                    _ => (PackageAvailability::Remote, None),
                };
                packages.push(Self {
                    availability,
                    details: package,
                    path,
                });
            }
        }
        Ok(packages)
    }

    pub(crate) fn into_package_info(self) -> PackageInfo {
        PackageInfo::new(
            self.availability,
            self.details.metadata().name(),
            self.details.url(),
            Version::new(self.details.metadata().index_version().version().clone()),
            Some(String::from(
                self.details
                    .metadata()
                    .index_version()
                    .release_group()
                    .as_str(),
            )),
            self.path,
        )
    }
}
