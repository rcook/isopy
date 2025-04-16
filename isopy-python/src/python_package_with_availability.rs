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
use crate::build_label::BuildLabel;
use crate::python_package::PythonPackage;
use isopy_lib::{Package, PackageAvailability, PackageInfo, Version};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct PythonPackageWithAvailability {
    pub(crate) package: PythonPackage,
    pub(crate) availability: PackageAvailability,
    pub(crate) path: Option<PathBuf>,
}

impl PythonPackageWithAvailability {
    pub(crate) fn into_package(self) -> Package {
        Package::new(self.package)
    }

    pub(crate) fn into_package_info(self) -> PackageInfo {
        let label = self
            .package
            .metadata()
            .version()
            .label()
            .as_ref()
            .map(BuildLabel::as_str);
        PackageInfo::new(
            self.availability,
            self.package.metadata().name(),
            self.package.url(),
            Version::new(self.package.metadata().version().clone()),
            label,
            self.path,
        )
    }
}
