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
use crate::base_version::BaseVersion;
use crate::build_label::BuildLabel;
use anyhow::{bail, Error, Result};
use isopy_lib::VersionOps;
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PythonVersion {
    version: BaseVersion,
    label: Option<BuildLabel>,
}

impl PythonVersion {
    pub(crate) fn from_tags(tags: &mut HashSet<String>) -> Result<Self> {
        let mut result = None;
        let mut version = None;
        let mut label = None;
        let mut tags_to_remove = Vec::new();

        for tag in tags.iter() {
            if let Some((prefix, suffix)) = tag.split_once('+') {
                if let Ok(temp_version) = BaseVersion::parse(prefix) {
                    if let Ok(temp_label) = suffix.parse() {
                        assert!(result.is_none() && version.is_none() && label.is_none());
                        tags_to_remove.push(tag.clone());
                        result = Some(Self {
                            version: temp_version,
                            label: Some(temp_label),
                        });
                        break;
                    }
                }
            }

            if let Ok(temp_version) = BaseVersion::parse(tag) {
                assert!(result.is_none() && version.is_none());
                tags_to_remove.push(tag.clone());
                version = Some(temp_version);
                if label.is_some() {
                    break;
                }
            }

            if let Ok(temp_label) = tag.parse() {
                assert!(result.is_none() && label.is_none());
                tags_to_remove.push(tag.clone());
                label = Some(temp_label);
                if version.is_some() {
                    break;
                }
            }
        }

        for tag in tags_to_remove {
            assert!(tags.remove(&tag));
        }

        if let Some(result) = result {
            assert!(version.is_none() && label.is_none());
            return Ok(result);
        }

        let Some(version) = version else {
            bail!("Could not determine package version from tags {tags:?}")
        };

        let Some(label) = label else {
            bail!("Could not determine package build label from tags {tags:?}")
        };

        Ok(Self {
            version,
            label: Some(label),
        })
    }

    pub(crate) const fn version(&self) -> &BaseVersion {
        &self.version
    }

    pub(crate) const fn label(&self) -> &Option<BuildLabel> {
        &self.label
    }

    pub(crate) fn matches(&self, other: &Self) -> bool {
        if self.version != *other.version() {
            return false;
        }

        if let Some(other_label) = other.label() {
            match &self.label {
                Some(label) if label == other_label => {}
                _ => return false,
            }
        }

        true
    }
}

impl Display for PythonVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.version)?;
        if let Some(label) = &self.label {
            write!(f, ":{}", label.as_str())?;
        }
        Ok(())
    }
}

impl FromStr for PythonVersion {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let (prefix, label) = match s.rsplit_once(':') {
            Some((prefix, suffix)) => (prefix, Some(suffix.parse()?)),
            None => (s, None),
        };

        let version = BaseVersion::parse(prefix)?;
        Ok(Self { version, label })
    }
}

impl VersionOps for PythonVersion {
    fn as_str(&self) -> Cow<String> {
        Cow::Owned(format!("{self}"))
    }

    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::PythonVersion;
    use crate::build_label::BuildLabel;
    use crate::discriminant::Discriminant;
    use crate::prerelease_kind::PrereleaseKind;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(1, 2, 3, Discriminant::None, None, "1.2.3")]
    #[case(
        1,
        2,
        3,
        Discriminant::prerelease(PrereleaseKind::ReleaseCandidate, 5),
        None,
        "1.2.3rc5"
    )]
    #[case(
        1,
        2,
        3,
        Discriminant::prerelease(PrereleaseKind::ReleaseCandidate, 2),
        Some("20250414".parse::<BuildLabel>().expect("Must succeed")),
        "1.2.3rc2:20250414"
    )]
    fn basics(
        #[case] expected_major: i32,
        #[case] expected_minor: i32,
        #[case] expected_revision: i32,
        #[case] expected_discriminant: Discriminant,
        #[case] expected_label: Option<BuildLabel>,
        #[case] input: &str,
    ) -> Result<()> {
        let result = input.parse::<PythonVersion>()?;
        let version = result.version();
        assert_eq!(expected_major, version.triple.major);
        assert_eq!(expected_minor, version.triple.minor);
        assert_eq!(expected_revision, version.triple.revision);
        assert_eq!(expected_discriminant, version.discriminant);
        assert_eq!(expected_label, result.label);
        Ok(())
    }
}
