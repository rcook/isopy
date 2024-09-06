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
use crate::tng::archive_group::ArchiveGroup;
use anyhow::Result;
use isopy_lib::tng::VersionTriple;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ArchiveFullVersion {
    pub(crate) version: VersionTriple,
    pub(crate) group: ArchiveGroup,
}

impl ArchiveFullVersion {
    pub(crate) fn from_tags(tags: &mut HashSet<String>) -> Result<Self> {
        let mut full_version = None;
        let mut version = None;
        let mut group = None;
        let mut tags_to_remove = Vec::new();

        for tag in tags.iter() {
            if let Some((prefix, suffix)) = tag.split_once('+') {
                if let Ok(temp_version) = prefix.parse() {
                    if let Ok(temp_group) = suffix.parse() {
                        assert!(full_version.is_none() && version.is_none() && group.is_none());
                        tags_to_remove.push(tag.clone());
                        full_version = Some(Self {
                            version: temp_version,
                            group: temp_group,
                        });
                        break;
                    }
                }
            }

            if let Ok(temp_version) = tag.parse() {
                assert!(full_version.is_none() && version.is_none());
                tags_to_remove.push(tag.clone());
                version = Some(temp_version);
                if group.is_some() {
                    break;
                }
            }

            if let Ok(temp_group) = tag.parse() {
                assert!(full_version.is_none() && group.is_none());
                tags_to_remove.push(tag.clone());
                group = Some(temp_group);
                if version.is_some() {
                    break;
                }
            }
        }

        for tag in tags_to_remove {
            assert!(tags.remove(&tag));
        }

        if let Some(result) = full_version {
            assert!(version.is_none() && group.is_none());
            return Ok(result);
        }

        let version = version.expect("Version must be found");
        let group = group.expect("Group must be found");
        Ok(Self { version, group })
    }
}
