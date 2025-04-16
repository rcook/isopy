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
use crate::{item::Item, metadata::Metadata};
use anyhow::Result;
use isopy_lib::{PackageOps, Version};
use url::Url;

#[derive(Clone, Debug)]
pub(crate) struct PythonPackage {
    url: Url,
    metadata: Metadata,
    version: Version,
}

impl PythonPackage {
    pub(crate) fn parse_all(item: &Item) -> Result<Vec<Self>> {
        macro_rules! g {
            ($e : expr) => {
                match $e {
                    Some(value) => value,
                    None => ::anyhow::bail!("Invalid index"),
                }
            };
        }

        fn filter_fn(name: &str) -> bool {
            name.starts_with("cpython-") && !name.ends_with(".sha256") && name != "SHA256SUMS"
        }

        let assets = g!(g!(item.value().get("assets")).as_array())
            .iter()
            .map(|asset| {
                let url = g!(g!(asset.get("browser_download_url")).as_str()).parse::<Url>()?;
                let name = g!(g!(asset.get("name")).as_str());
                Ok((url, name))
            })
            .collect::<Result<Vec<_>>>()?;
        let packages = assets
            .into_iter()
            .filter(|(_, name)| filter_fn(name))
            .map(|(url, name)| {
                let metadata = name.parse::<Metadata>()?;
                let package = Self::new(&url, metadata);
                Ok(package)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(packages)
    }

    pub(crate) fn new(url: &Url, metadata: Metadata) -> Self {
        let version = Version::new(metadata.version.clone());
        Self {
            url: url.clone(),
            metadata,
            version,
        }
    }

    pub(crate) const fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl PackageOps for PythonPackage {
    fn version(&self) -> &Version {
        &self.version
    }

    fn url(&self) -> &Url {
        self.url()
    }
}
