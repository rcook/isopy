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
use isopy_lib::{ArchiveType, Checksum, PackageAvailability, PackageOps, Version};
use std::{collections::HashSet, path::PathBuf};
use url::Url;

#[allow(unused)]
pub(crate) struct JavaPackage {
    name: String,
    availability: PackageAvailability,
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
        availability: PackageAvailability,
        archive_type: ArchiveType,
        url: &Url,
        version: &JavaVersion,
        path: &Option<PathBuf>,
        checksum: Checksum,
        tags: Vec<String>,
    ) -> Self {
        let version = version.clone();
        let other_version = Version::new(version.clone());
        let tags = tags.into_iter().collect::<HashSet<_>>();
        Self {
            name: String::from(name),
            availability,
            archive_type,
            url: url.clone(),
            version,
            other_version,
            path: path.clone(),
            checksum,
            tags,
        }
    }

    #[allow(unused)]
    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    #[allow(unused)]
    pub(crate) const fn kind(&self) -> &PackageAvailability {
        &self.availability
    }

    #[allow(unused)]
    pub(crate) const fn archive_type(&self) -> &ArchiveType {
        &self.archive_type
    }

    #[allow(unused)]
    pub(crate) const fn url(&self) -> &Url {
        &self.url
    }

    #[allow(unused)]
    pub(crate) const fn version(&self) -> &JavaVersion {
        &self.version
    }

    #[allow(unused)]
    pub(crate) const fn path(&self) -> &Option<PathBuf> {
        &self.path
    }

    #[allow(unused)]
    pub(crate) const fn checksum(&self) -> &Checksum {
        &self.checksum
    }

    #[allow(unused)]
    pub(crate) const fn tags(&self) -> &HashSet<String> {
        &self.tags
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
