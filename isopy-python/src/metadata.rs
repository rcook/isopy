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
use crate::archive_type::ArchiveType;
use crate::full_version::FullVersion;
use anyhow::{bail, Error, Result};
use std::collections::HashSet;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub(crate) struct Metadata {
    name: String,
    archive_type: ArchiveType,
    full_version: FullVersion,
    tags: HashSet<String>,
}

impl Metadata {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn archive_type(&self) -> &ArchiveType {
        &self.archive_type
    }

    pub(crate) fn full_version(&self) -> &FullVersion {
        &self.full_version
    }

    pub(crate) fn tags(&self) -> &HashSet<String> {
        &self.tags
    }
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_archive_type(s: &str) -> Result<(&str, ArchiveType)> {
            for archive_type in ArchiveType::iter() {
                if let Some(prefix) = s.strip_suffix(archive_type.suffix()) {
                    return Ok((prefix, archive_type));
                }
            }
            bail!("Archive {s} is not a valid Python archive")
        }

        let name = String::from(s);

        let (prefix, archive_type) = parse_archive_type(s)?;

        let mut tags = prefix.split('-').map(str::to_owned).collect::<HashSet<_>>();
        if !tags.remove("cpython") {
            bail!("Archive {s} is not a valid Python archive")
        }

        let full_version = FullVersion::from_tags(&mut tags)?;

        Ok(Self {
            name,
            archive_type,
            full_version,
            tags,
        })
    }
}
