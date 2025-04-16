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
use crate::build_label::BuildLabel;
use crate::discriminant::Discriminant;
use crate::prerelease_kind::PrereleaseKind;
use anyhow::{bail, Error, Result};
use isopy_lib::Triple;
use isopy_lib::VersionOps;
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct PythonVersion {
    pub(crate) triple: Triple,
    pub(crate) discriminant: Discriminant,
    pub(crate) label: Option<BuildLabel>,
}

fn parse_triple_discriminant_helper(s: &str) -> Result<(Triple, Discriminant)> {
    fn prerelease_helper(
        prerelease_type: PrereleaseKind,
        s: &str,
        i0: usize,
        i1: usize,
    ) -> Result<(Triple, Discriminant)> {
        let triple_str = &s[..i0];
        let triple = triple_str.parse()?;
        let number_str = &s[i1..];
        let number = number_str.parse()?;
        let discriminant = Discriminant::prerelease(prerelease_type, number);
        Ok((triple, discriminant))
    }
    if let Some(i) = s.find('a') {
        return prerelease_helper(PrereleaseKind::Alpha, s, i, i + 1);
    }
    if let Some(i) = s.find("rc") {
        return prerelease_helper(PrereleaseKind::ReleaseCandidate, s, i, i + 2);
    }

    let triple = s.parse()?;
    Ok((triple, Discriminant::None))
}

impl PythonVersion {
    pub(crate) fn from_tags(tags: &mut HashSet<String>) -> Result<Self> {
        let mut result = None;
        let mut triple_discriminant = None;
        let mut label = None;
        let mut tags_to_remove = Vec::new();

        for tag in tags.iter() {
            if let Some((prefix, suffix)) = tag.split_once('+') {
                if let Ok(temp_triple_discriminant) = parse_triple_discriminant_helper(prefix) {
                    if let Ok(temp_label) = suffix.parse() {
                        assert!(
                            result.is_none() && triple_discriminant.is_none() && label.is_none()
                        );
                        tags_to_remove.push(tag.clone());
                        result = Some(Self {
                            triple: temp_triple_discriminant.0,
                            discriminant: temp_triple_discriminant.1,
                            label: Some(temp_label),
                        });
                        break;
                    }
                }
            }

            if let Ok(temp_triple_discriminant) = parse_triple_discriminant_helper(tag) {
                assert!(result.is_none() && triple_discriminant.is_none());
                tags_to_remove.push(tag.clone());
                triple_discriminant = Some(temp_triple_discriminant);
                if label.is_some() {
                    break;
                }
            }

            if let Ok(temp_label) = tag.parse() {
                assert!(result.is_none() && label.is_none());
                tags_to_remove.push(tag.clone());
                label = Some(temp_label);
                if triple_discriminant.is_some() {
                    break;
                }
            }
        }

        for tag in tags_to_remove {
            assert!(tags.remove(&tag));
        }

        if let Some(result) = result {
            assert!(triple_discriminant.is_none() && label.is_none());
            return Ok(result);
        }

        let Some(triple_discriminant) = triple_discriminant else {
            bail!("Could not determine package version from tags {tags:?}")
        };

        let Some(label) = label else {
            bail!("Could not determine package build label from tags {tags:?}")
        };

        Ok(Self {
            triple: triple_discriminant.0,
            discriminant: triple_discriminant.1,
            label: Some(label),
        })
    }

    pub(crate) fn matches(&self, other: &Self) -> bool {
        if self.triple != other.triple {
            return false;
        }
        if self.discriminant != other.discriminant {
            return false;
        }

        if let Some(other_label) = other.label() {
            match &self.label {
                Some(label) if label.as_str() == other_label.as_str() => {}
                _ => return false,
            }
        }

        true
    }
}

impl Display for PythonVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())?;
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

        let (triple, discriminant) = parse_triple_discriminant_helper(prefix)?;
        Ok(Self {
            triple,
            discriminant,
            label,
        })
    }
}

impl VersionOps for PythonVersion {
    fn as_str(&self) -> Cow<String> {
        match &self.discriminant {
            Discriminant::Prerelease(d) => Cow::Owned(format!("{}{}", self.triple, d)),
            Discriminant::None => Cow::Owned(self.triple.to_string()),
        }
    }

    fn label(&self) -> Option<Cow<String>> {
        self.label
            .as_ref()
            .map(|l| Cow::Owned(String::from(l.as_str())))
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
    use super::parse_triple_discriminant_helper;
    use super::PythonVersion;
    use crate::build_label::BuildLabel;
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
    fn parse_triple_discriminant_helper_basics(
        #[case] expected_major: i32,
        #[case] expected_minor: i32,
        #[case] expected_revision: i32,
        #[case] expected_discriminant: Discriminant,
        #[case] input: &str,
    ) -> Result<()> {
        let (triple, discriminant) = parse_triple_discriminant_helper(input)?;
        assert_eq!(expected_major, triple.major);
        assert_eq!(expected_minor, triple.minor);
        assert_eq!(expected_revision, triple.revision);
        assert_eq!(expected_discriminant, discriminant);
        //assert_eq!(input, result.to_string());
        Ok(())
    }

    #[test]
    fn invalid() {
        assert!(parse_triple_discriminant_helper("3.14.0a10+20250409").is_err());
    }

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
        assert_eq!(expected_major, result.triple.major);
        assert_eq!(expected_minor, result.triple.minor);
        assert_eq!(expected_revision, result.triple.revision);
        assert_eq!(expected_discriminant, result.discriminant);
        assert_eq!(expected_label, result.label);
        Ok(())
    }
}
