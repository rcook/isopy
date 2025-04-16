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
use anyhow::{anyhow, bail, Error};
use isopy_lib::ArchiveType;
use std::collections::HashSet;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) struct Metadata {
    name: String,
    archive_type: ArchiveType,
    version: PythonVersion,
    tags: HashSet<String>,
}

impl Metadata {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn archive_type(&self) -> &ArchiveType {
        &self.archive_type
    }

    pub(crate) const fn version(&self) -> &PythonVersion {
        &self.version
    }

    pub(crate) const fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    #[allow(unused)]
    pub(crate) fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let name = String::from(s);

        let (archive_type, prefix) = ArchiveType::strip_suffix(s)
            .ok_or_else(|| anyhow!("Cannot determine archive type of file name {s}"))?;

        let mut tags = prefix.split('-').map(str::to_owned).collect::<HashSet<_>>();
        if !tags.remove("cpython") {
            bail!("Archive {s} is not a valid Python archive")
        }

        let version = PythonVersion::from_tags(&mut tags)?;
        if let Some(label) = version.label() {
            tags.insert(String::from(label.as_str()));
        }

        Ok(Self {
            name,
            archive_type,
            version,
            tags,
        })
    }
}
