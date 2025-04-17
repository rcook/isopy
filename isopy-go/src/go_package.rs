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
use isopy_lib::{ArchiveType, Checksum, PackageInfo, PackageOps, Version};
use std::{collections::HashSet, path::PathBuf};
use url::Url;

pub(crate) struct GoPackage {
    pub(crate) name: String,
    pub(crate) archive_type: ArchiveType,
    pub(crate) url: Url,
    pub(crate) version: GoVersion,
    pub(crate) other_version: Version,
    pub(crate) path: Option<PathBuf>,
    pub(crate) checksum: Checksum,
    pub(crate) tags: HashSet<String>,
}

impl GoPackage {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        name: &str,
        archive_type: ArchiveType,
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
            archive_type,
            url: url.clone(),
            version,
            other_version,
            path: path.clone(),
            checksum,
            tags,
        }
    }

    pub(crate) fn into_package_info(self) -> PackageInfo {
        let version = PackageOps::version(&self).clone();
        PackageInfo::new(self.name, &self.url, version, self.path)
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
