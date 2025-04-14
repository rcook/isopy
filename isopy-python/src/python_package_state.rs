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
use crate::release_group::ReleaseGroup;
use anyhow::Result;
use isopy_lib::{Package, PackageAvailability, PackageManagerContext, PackageState, Version};
use std::collections::HashSet;
use std::path::PathBuf;

pub(crate) struct PythonPackageState {
    availability: PackageAvailability,
    package: PythonPackage,
    path: Option<PathBuf>,
}

impl PythonPackageState {
    pub(crate) async fn read(
        ctx: &PackageManagerContext,
        index: &Index,
        version: &PythonVersion,
        tags: &HashSet<&str>,
    ) -> Result<Option<Self>> {
        let mut packages = Vec::new();
        for item in index.items() {
            for package in PythonPackage::parse_all(&item)? {
                let m = package.metadata();
                if m.has_tags(tags) && m.version().matches(version) {
                    let (availability, path) = match ctx.get_file(package.url()).await {
                        Ok(p) => (PackageAvailability::Local, Some(p)),
                        _ => (PackageAvailability::Remote, None),
                    };
                    packages.push(Self {
                        availability,
                        package,
                        path,
                    });
                }
            }
        }

        packages.sort_by_cached_key(|p| p.package.metadata().version().clone());
        packages.reverse();
        Ok(packages.into_iter().next())
    }

    pub(crate) async fn read_all(ctx: &PackageManagerContext, index: &Index) -> Result<Vec<Self>> {
        let mut packages = Vec::new();
        for item in index.items() {
            for package in PythonPackage::parse_all(&item)? {
                let (availability, path) = match ctx.get_file(package.url()).await {
                    Ok(p) => (PackageAvailability::Local, Some(p)),
                    _ => (PackageAvailability::Remote, None),
                };
                packages.push(Self {
                    availability,
                    package,
                    path,
                });
            }
        }
        Ok(packages)
    }

    pub(crate) const fn availability(&self) -> PackageAvailability {
        self.availability
    }

    pub(crate) const fn package(&self) -> &PythonPackage {
        &self.package
    }

    pub(crate) const fn path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub(crate) fn into_package(self) -> Package {
        Package::new(self.package)
    }

    pub(crate) fn into_package_state(self) -> PackageState {
        let label = self
            .package
            .metadata()
            .version()
            .release_group()
            .as_ref()
            .map(ReleaseGroup::as_str);
        PackageState::new(
            self.availability,
            self.package.metadata().name(),
            self.package.url(),
            Version::new(self.package.metadata().version().version().clone()),
            label,
            self.path,
        )
    }
}
