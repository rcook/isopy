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
use crate::python_package_with_availability::PythonPackageWithAvailability;
use itertools::Itertools;

// On some platforms, most notably Windows, we will get more than one package
// matching the given tags: these functions will choose the "best" so that
// there is exactly one matching package for a given version and build label

#[cfg(not(target_os = "windows"))]
pub(crate) fn uniquify_packages(
    packages: Vec<PythonPackageWithAvailability>,
) -> Vec<PythonPackageWithAvailability> {
    for (key, group) in &packages
        .iter()
        .chunk_by(|p| p.package.metadata.version.clone())
    {
        assert_eq!(
            1,
            group.count(),
            "More than one viable candidate for package {key}"
        );
    }
    packages
}

// On Windows, we prefer the "shared" library over the "static" library and
// choose the default otherwise
#[cfg(target_os = "windows")]
pub(crate) fn uniquify_packages(
    packages: Vec<PythonPackageWithAvailability>,
) -> Vec<PythonPackageWithAvailability> {
    let mut filtered_packages = Vec::new();
    for (key, group) in &packages
        .into_iter()
        .chunk_by(|p| p.package.metadata().version.clone())
    {
        let packages = group.collect::<Vec<_>>();
        let package_count = packages.len();
        assert_ne!(0, package_count);

        if package_count > 1 {
            let mut temp_shared = None;
            let mut temp_static = None;
            let mut temp_default = None;

            for package in packages {
                if package.package.metadata().has_tag("shared") {
                    assert!(temp_shared.is_none());
                    temp_shared = Some(package);
                } else if package.package.metadata().has_tag("static") {
                    assert!(temp_static.is_none());
                    temp_static = Some(package);
                } else {
                    assert!(temp_default.is_none());
                    temp_default = Some(package);
                }
            }

            assert!(
                temp_shared.is_some() || temp_default.is_some() || temp_static.is_some(),
                "No viable candidate for package {key}"
            );

            if let Some(package) = temp_shared {
                filtered_packages.push(package);
            } else if let Some(package) = temp_default {
                filtered_packages.push(package);
            } else if let Some(package) = temp_static {
                filtered_packages.push(package);
            } else {
                unreachable!()
            }
        } else {
            let package = packages
                .into_iter()
                .next()
                .expect("Must have exactly one element");
            filtered_packages.push(package);
        }
    }

    filtered_packages
}
