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
use crate::metadata::Metadata;
use crate::python_package::PythonPackage;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct Container {
    #[serde(rename = "packages")]
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "url")]
    url: Url,

    #[serde(rename = "version")]
    version: String,
}

pub(crate) fn write_package_cache(path: &Path, packages: &[PythonPackage]) -> Result<()> {
    fn transform(package: &PythonPackage) -> Package {
        let m = package.metadata();
        Package {
            name: m.name.clone(),
            url: package.url().clone(),
            version: m.version.to_string(),
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

pub(crate) fn read_package_cache(path: &Path) -> Result<Vec<PythonPackage>> {
    fn transform(package: &Package) -> Result<PythonPackage> {
        let metadata = package.name.parse::<Metadata>()?;
        Ok(PythonPackage::new(&package.url, metadata))
    }

    let f = File::open(path)?;
    let container = serde_yaml::from_reader::<_, Container>(f)?;

    container
        .packages
        .iter()
        .map(transform)
        .collect::<Result<Vec<_>>>()
}
