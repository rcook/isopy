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
use crate::go_version::GoVersion;
use isopy_lib::{Checksum, PackageKind, PackageOps, Version};
use std::{collections::HashSet, path::PathBuf};
use url::Url;

pub(crate) struct GoPackage {
    name: String,
    kind: PackageKind,
    url: Url,
    version: GoVersion,
    other_version: Version,
    path: Option<PathBuf>,
    checksum: Checksum,
    tags: HashSet<String>,
}

impl GoPackage {
    pub(crate) fn new(
        name: &str,
        kind: PackageKind,
        url: &Url,
        version: &GoVersion,
        path: &Option<PathBuf>,
        checksum: Checksum,
        tags: Vec<String>,
    ) -> Self {
        let version = version.clone();
        let other_version = Version::new(version.clone());
        let tags = tags.into_iter().collect::<HashSet<_>>();
        Self {
            name: String::from(name),
            kind,
            url: url.clone(),
            version,
            other_version,
            path: path.clone(),
            checksum,
            tags,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(crate) const fn kind(&self) -> &PackageKind {
        &self.kind
    }

    pub(crate) const fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) const fn version(&self) -> &GoVersion {
        &self.version
    }

    pub(crate) const fn path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub(crate) const fn checksum(&self) -> &Checksum {
        &self.checksum
    }

    pub(crate) const fn tags(&self) -> &HashSet<String> {
        &self.tags
    }
}

impl PackageOps for GoPackage {
    fn version(&self) -> &Version {
        &self.other_version
    }

    fn url(&self) -> &Url {
        &self.url
    }
}
