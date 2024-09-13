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
use crate::build_tag::BuildTag;
use anyhow::{bail, Result};
use isopy_lib::VersionTriple;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct FullVersion {
    version: VersionTriple,
    version_ex: Option<String>,
    build_tag: BuildTag,
}

impl FullVersion {
    pub(crate) fn from_tags(tags: &mut HashSet<String>) -> Result<Self> {
        fn get_version_and_version_ex(prefix: &str) -> (&str, Option<String>) {
            prefix.find("rc").map_or_else(
                || (prefix, None),
                |i| (&prefix[0..i], Some(String::from(&prefix[i..]))),
            )
        }

        let mut full_version = None;
        let mut version = None;
        let mut version_ex = None;
        let mut build_tag = None;
        let mut tags_to_remove = Vec::new();

        for tag in tags.iter() {
            if let Some((prefix, suffix)) = tag.split_once('+') {
                let (version_str, temp_version_ex) = get_version_and_version_ex(prefix);
                if let Ok(temp_version) = version_str.parse() {
                    if let Ok(temp_build_tag) = suffix.parse() {
                        assert!(
                            full_version.is_none()
                                && version.is_none()
                                && version_ex.is_none()
                                && build_tag.is_none()
                        );
                        tags_to_remove.push(tag.clone());
                        full_version = Some(Self {
                            version: temp_version,
                            version_ex: temp_version_ex,
                            build_tag: temp_build_tag,
                        });
                        break;
                    }
                }
            }

            let (version_str, temp_version_ex) = get_version_and_version_ex(tag);
            if let Ok(temp_version) = version_str.parse() {
                assert!(full_version.is_none() && version.is_none() && version_ex.is_none());
                tags_to_remove.push(tag.clone());
                version = Some(temp_version);
                version_ex = temp_version_ex;
                if build_tag.is_some() {
                    break;
                }
            }

            if let Ok(temp_build_tag) = tag.parse() {
                assert!(full_version.is_none() && build_tag.is_none());
                tags_to_remove.push(tag.clone());
                build_tag = Some(temp_build_tag);
                if version.is_some() {
                    break;
                }
            }
        }

        for tag in tags_to_remove {
            assert!(tags.remove(&tag));
        }

        if let Some(result) = full_version {
            assert!(version.is_none() && build_tag.is_none());
            return Ok(result);
        }

        let Some(version) = version else {
            bail!("Could not determine package version from tags {tags:?}")
        };

        let Some(build_tag) = build_tag else {
            bail!("Could not determine package build tag from tags {tags:?}")
        };

        Ok(Self {
            version,
            version_ex: None,
            build_tag,
        })
    }

    pub(crate) const fn version(&self) -> &VersionTriple {
        &self.version
    }

    pub(crate) const fn build_tag(&self) -> &BuildTag {
        &self.build_tag
    }
}
