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
use crate::release_group::ReleaseGroup;
use crate::version_with_discriminant::VersionWithDiscriminant;
use anyhow::{bail, Result};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PythonIndexVersion {
    version: VersionWithDiscriminant,
    release_group: ReleaseGroup,
}

impl PythonIndexVersion {
    pub(crate) fn from_tags(tags: &mut HashSet<String>) -> Result<Self> {
        let mut result = None;
        let mut version = None;
        let mut release_group = None;
        let mut tags_to_remove = Vec::new();

        for tag in tags.iter() {
            if let Some((prefix, suffix)) = tag.split_once('+') {
                if let Ok(temp_version) = VersionWithDiscriminant::parse(prefix) {
                    if let Ok(temp_release_group) = suffix.parse() {
                        assert!(result.is_none() && version.is_none() && release_group.is_none());
                        tags_to_remove.push(tag.clone());
                        result = Some(Self {
                            version: temp_version,
                            release_group: temp_release_group,
                        });
                        break;
                    }
                }
            }

            if let Ok(temp_version) = VersionWithDiscriminant::parse(tag) {
                assert!(result.is_none() && version.is_none());
                tags_to_remove.push(tag.clone());
                version = Some(temp_version);
                if release_group.is_some() {
                    break;
                }
            }

            if let Ok(temp_release_group) = tag.parse() {
                assert!(result.is_none() && release_group.is_none());
                tags_to_remove.push(tag.clone());
                release_group = Some(temp_release_group);
                if version.is_some() {
                    break;
                }
            }
        }

        for tag in tags_to_remove {
            assert!(tags.remove(&tag));
        }

        if let Some(result) = result {
            assert!(version.is_none() && release_group.is_none());
            return Ok(result);
        }

        let Some(version) = version else {
            bail!("Could not determine package version from tags {tags:?}")
        };

        let Some(release_group) = release_group else {
            bail!("Could not determine package release group from tags {tags:?}")
        };

        Ok(Self {
            version,
            release_group,
        })
    }

    pub(crate) const fn version(&self) -> &VersionWithDiscriminant {
        &self.version
    }

    pub(crate) const fn release_group(&self) -> &ReleaseGroup {
        &self.release_group
    }

    pub(crate) fn matches(&self, other: &PythonVersion) -> bool {
        if self.version != *other.version() {
            return false;
        }

        if let Some(other_release_group) = other.release_group() {
            if self.release_group != *other_release_group {
                return false;
            }
        }

        true
    }
}

// Could not determine package version from tags {"aarch64", "debug", "full", "3.14.0a6+20250409", "darwin", "apple"}

#[cfg(test)]
mod tests {
    use super::PythonIndexVersion;
    use crate::discriminant::Discriminant;
    use crate::prerelease_type::PrereleaseType;
    use anyhow::Result;
    use isopy_lib::VersionTriple;
    use std::collections::HashSet;

    #[test]
    fn basics() -> Result<()> {
        let mut tags = vec![
            "aarch64",
            "debug",
            "full",
            "3.14.0a6+20250409",
            "darwin",
            "apple",
        ]
        .into_iter()
        .map(String::from)
        .collect::<HashSet<_>>();
        let result = PythonIndexVersion::from_tags(&mut tags)?;
        assert_eq!(
            VersionTriple {
                major: 3,
                minor: 14,
                revision: 0
            },
            result.version.version
        );
        assert_eq!(
            Discriminant::prerelease(PrereleaseType::Alpha, 6),
            result.version.discriminant
        );
        assert_eq!("20250409", result.release_group.as_str());
        Ok(())
    }
}
