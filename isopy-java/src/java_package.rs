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
use crate::java_version::JavaVersion;
use isopy_lib::{ArchiveType, Checksum, PackageOps, Version};
use std::{collections::HashSet, path::PathBuf};
use url::Url;

#[allow(unused)]
pub struct JavaPackage {
    name: String,
    archive_type: ArchiveType,
    url: Url,
    version: JavaVersion,
    other_version: Version,
    path: Option<PathBuf>,
    checksum: Checksum,
    tags: HashSet<String>,
}

impl JavaPackage {
    #[allow(unused)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        name: &str,
        archive_type: ArchiveType,
        url: &Url,
        version: &JavaVersion,
        path: Option<&PathBuf>,
        checksum: Checksum,
        tags: Vec<String>,
    ) -> Self {
        let version = version.clone();
        let other_version = Version::new(version.clone());
        let tags = tags.into_iter().collect::<HashSet<_>>();
        Self {
            name: String::from(name),
            archive_type,
            url: url.clone(),
            version,
            other_version,
            path: path.cloned(),
            checksum,
            tags,
        }
    }
}

impl PackageOps for JavaPackage {
    fn version(&self) -> &Version {
        &self.other_version
    }

    fn url(&self) -> &Url {
        &self.url
    }
}
