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
use crate::release_group::ReleaseGroup;
use anyhow::Error;
use isopy_lib::VersionOps;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ProjectVersion {
    version: BaseVersion,
    release_group: Option<ReleaseGroup>,
}

impl ProjectVersion {
    pub(crate) const fn version(&self) -> &BaseVersion {
        &self.version
    }

    pub(crate) const fn release_group(&self) -> &Option<ReleaseGroup> {
        &self.release_group
    }
}

impl Display for ProjectVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.version)?;
        if let Some(release_group) = &self.release_group {
            write!(f, ":{}", release_group.as_str())?;
        }
        Ok(())
    }
}

impl FromStr for ProjectVersion {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let (prefix, release_group) = match s.rsplit_once(':') {
            Some((prefix, suffix)) => (prefix, Some(suffix.parse()?)),
            None => (s, None),
        };

        let version = BaseVersion::parse(prefix)?;
        Ok(Self {
            version,
            release_group,
        })
    }
}

impl VersionOps for ProjectVersion {
    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectVersion;
    use crate::discriminant::Discriminant;
    use crate::prerelease_kind::PrereleaseKind;
    use crate::release_group::ReleaseGroup;
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
        Some("20250414".parse::<ReleaseGroup>().expect("Must succeed")),
        "1.2.3rc2:20250414"
    )]
    fn basics(
        #[case] expected_major: i32,
        #[case] expected_minor: i32,
        #[case] expected_revision: i32,
        #[case] expected_discriminant: Discriminant,
        #[case] expected_release_group: Option<ReleaseGroup>,
        #[case] input: &str,
    ) -> Result<()> {
        let result = input.parse::<ProjectVersion>()?;
        let version = result.version();
        assert_eq!(expected_major, version.triple.major);
        assert_eq!(expected_minor, version.triple.minor);
        assert_eq!(expected_revision, version.triple.revision);
        assert_eq!(expected_discriminant, version.discriminant);
        assert_eq!(expected_release_group, result.release_group);
        Ok(())
    }
}
