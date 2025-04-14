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
use crate::discriminant::Discriminant;
use crate::prerelease_kind::PrereleaseKind;
use anyhow::Result;
use isopy_lib::{Triple, VersionOps};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct BaseVersion {
    pub(crate) triple: Triple,
    pub(crate) discriminant: Discriminant,
}

impl BaseVersion {
    pub(crate) fn parse(s: &str) -> Result<Self> {
        fn prerelease_helper(
            prerelease_type: PrereleaseKind,
            s: &str,
            i0: usize,
            i1: usize,
        ) -> Result<BaseVersion> {
            let version_str = &s[..i0];
            let version = version_str.parse()?;
            let number_str = &s[i1..];
            let number = number_str.parse()?;
            let discriminant = Discriminant::prerelease(prerelease_type, number);
            Ok(BaseVersion {
                triple: version,
                discriminant,
            })
        }
        if let Some(i) = s.find('a') {
            return prerelease_helper(PrereleaseKind::Alpha, s, i, i + 1);
        }
        if let Some(i) = s.find("rc") {
            return prerelease_helper(PrereleaseKind::ReleaseCandidate, s, i, i + 2);
        }

        let version = s.parse()?;
        Ok(Self {
            triple: version,
            discriminant: Discriminant::None,
        })
    }
}

impl Display for BaseVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.discriminant {
            Discriminant::Prerelease(d) => write!(f, "{}{}", self.triple, d),
            _ => write!(f, "{}", self.triple),
        }
    }
}

impl VersionOps for BaseVersion {
    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_arguments)]

    use super::BaseVersion;
    use crate::discriminant::Discriminant;
    use crate::prerelease_kind::PrereleaseKind;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(
        3,
        14,
        0,
        Discriminant::prerelease(PrereleaseKind::Alpha, 10),
        "3.14.0a10"
    )]
    #[case(
        3,
        14,
        0,
        Discriminant::prerelease(PrereleaseKind::Alpha, 6),
        "3.14.0a6"
    )]
    #[case(
        3,
        14,
        0,
        Discriminant::prerelease(PrereleaseKind::ReleaseCandidate, 10),
        "3.14.0rc10"
    )]
    #[case(
        3,
        14,
        123,
        Discriminant::prerelease(PrereleaseKind::ReleaseCandidate, 345),
        "3.14.123rc345"
    )]
    fn basics(
        #[case] expected_major: i32,
        #[case] expected_minor: i32,
        #[case] expected_revision: i32,
        #[case] expected_discriminant: Discriminant,
        #[case] input: &str,
    ) -> Result<()> {
        let result = BaseVersion::parse(input)?;
        assert_eq!(expected_major, result.triple.major);
        assert_eq!(expected_minor, result.triple.minor);
        assert_eq!(expected_revision, result.triple.revision);
        assert_eq!(expected_discriminant, result.discriminant);
        assert_eq!(input, result.to_string());
        Ok(())
    }

    #[test]
    fn invalid() {
        assert!(BaseVersion::parse("3.14.0a10+20250409").is_err());
    }
}
