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
use crate::python_version::PythonVersion;
use anyhow::Result;
use isopy_lib::{PackageAvailability, PackageInfo, Version};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct Container {
    #[serde(rename = "packages")]
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    #[serde(rename = "availability")]
    availability: PackageAvailability,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "url")]
    url: Url,

    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    path: Option<PathBuf>,
}

pub(crate) fn write_package_cache(path: &Path, packages: &Vec<PackageInfo>) -> Result<()> {
    fn transform(p: &PackageInfo) -> Package {
        Package {
            availability: p.availability(),
            name: String::from(p.name()),
            url: p.url().clone(),
            version: p.version().to_string(),
            label: p.label().clone(),
            path: p.path().clone(),
        }
    }

    let f = File::create(path)?;
    serde_yaml::to_writer(
        f,
        &Container {
            packages: packages.iter().map(transform).collect::<Vec<_>>(),
        },
    )?;

    Ok(())
}

pub(crate) fn read_package_cache(path: &Path) -> Result<Vec<PackageInfo>> {
    fn transform(p: &Package) -> Result<PackageInfo> {
        let version = Version::new(p.version.parse::<PythonVersion>()?);
        Ok(PackageInfo::new(
            p.availability,
            &p.name,
            &p.url,
            version,
            p.label.clone(),
            p.path.clone(),
        ))
    }

    let f = File::open(path)?;
    let container = serde_yaml::from_reader::<_, Container>(f)?;

    Ok(container
        .packages
        .iter()
        .map(transform)
        .collect::<Result<Vec<_>>>()?)
}
