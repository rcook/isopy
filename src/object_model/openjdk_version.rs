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
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenJdkVersionParseError {
    #[error("invalid format {0}")]
    InvalidFormat(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type OpenJdkVersionParseResult<T> = StdResult<T, OpenJdkVersionParseError>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenJdkVersionKind {
    V1,
    V2,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OpenJdkVersion {
    pub raw: String,
    pub kind: OpenJdkVersionKind,
    pub major: u32,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub build: Option<u32>,
    pub qualifier1: u32,
    pub qualifier2: Option<String>,
}

type VersionQuad = (u32, Option<u32>, Option<u32>, Option<u32>);

impl OpenJdkVersion {
    fn to_u32(s: &str) -> OpenJdkVersionParseResult<u32> {
        s.parse::<u32>()
            .map_err(|e| OpenJdkVersionParseError::Other(anyhow!(e)))
    }

    fn parse_dotted(input: &str, s: &str) -> OpenJdkVersionParseResult<VersionQuad> {
        let parts = s.split('.').collect::<Vec<_>>();
        match parts.len() {
            1 => Ok((Self::to_u32(parts[0])?, None, None, None)),
            2 => Ok((
                Self::to_u32(parts[0])?,
                Some(Self::to_u32(parts[1])?),
                None,
                None,
            )),
            3 => Ok((
                Self::to_u32(parts[0])?,
                Some(Self::to_u32(parts[1])?),
                Some(Self::to_u32(parts[2])?),
                None,
            )),
            4 => Ok((
                Self::to_u32(parts[0])?,
                Some(Self::to_u32(parts[1])?),
                Some(Self::to_u32(parts[2])?),
                Some(Self::to_u32(parts[3])?),
            )),
            _ => Err(OpenJdkVersionParseError::InvalidFormat(String::from(input))),
        }
    }
}

impl FromStr for OpenJdkVersion {
    type Err = OpenJdkVersionParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if let Some((prefix, suffix)) = s.split_once('+') {
            let (major, minor, patch, build) = Self::parse_dotted(s, prefix)?;
            let qualifier1 = Self::to_u32(suffix)?;
            return Ok(Self {
                raw: String::from(s),
                kind: OpenJdkVersionKind::V2,
                major,
                minor,
                patch,
                build,
                qualifier1,
                qualifier2: None,
            });
        }

        if let Some((prefix, suffix)) = s.split_once('_') {
            let (major, minor, patch, build) = Self::parse_dotted(s, prefix)?;
            let Some((q1, q2)) = suffix.split_once('-') else {
                return Err(OpenJdkVersionParseError::InvalidFormat(String::from(s)))
            };
            let qualifier1 = Self::to_u32(q1)?;
            let qualifier2 = Some(String::from(q2));
            return Ok(Self {
                raw: String::from(s),
                kind: OpenJdkVersionKind::V1,
                major,
                minor,
                patch,
                build,
                qualifier1,
                qualifier2,
            });
        }

        todo!("(1): {s}")
    }
}

impl Display for OpenJdkVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.raw)
    }
}

impl<'de> Deserialize<'de> for OpenJdkVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for OpenJdkVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.raw)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_arguments)]
    use super::OpenJdkVersionKind::{V1, V2};
    use super::{OpenJdkVersion, OpenJdkVersionKind, OpenJdkVersionParseResult};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(V2, 20, Some(0), Some(1), None, 9, None, "20.0.1+9")]
    #[case(V2, 20, None, None, None, 36, None, "20+36")]
    #[case(V2, 19, Some(0), Some(2), None, 7, None, "19.0.2+7")]
    #[case(V2, 19, Some(0), Some(1), None, 10, None, "19.0.1+10")]
    #[case(V2, 19, None, None, None, 36, None, "19+36")]
    #[case(V2, 18, Some(0), Some(2), Some(1), 1, None, "18.0.2.1+1")]
    #[case(V2, 18, Some(0), Some(2), None, 9, None, "18.0.2+9")]
    #[case(V2, 18, Some(0), Some(1), None, 10, None, "18.0.1+10")]
    #[case(V2, 17, Some(0), Some(7), None, 7, None, "17.0.7+7")]
    #[case(V2, 17, Some(0), Some(6), None, 10, None, "17.0.6+10")]
    #[case(V2, 17, Some(0), Some(5), None, 8, None, "17.0.5+8")]
    #[case(V2, 17, Some(0), Some(4), Some(1), 1, None, "17.0.4.1+1")]
    #[case(V2, 17, Some(0), Some(4), None, 8, None, "17.0.4+8")]
    #[case(V2, 17, Some(0), Some(3), None, 7, None, "17.0.3+7")]
    #[case(V2, 17, Some(0), Some(2), None, 8, None, "17.0.2+8")]
    #[case(V2, 17, Some(0), Some(1), None, 12, None, "17.0.1+12")]
    #[case(V2, 11, Some(0), Some(19), None, 7, None, "11.0.19+7")]
    #[case(V2, 11, Some(0), Some(18), None, 10, None, "11.0.18+10")]
    #[case(V2, 11, Some(0), Some(17), None, 8, None, "11.0.17+8")]
    #[case(V2, 11, Some(0), Some(16), Some(1), 1, None, "11.0.16.1+1")]
    #[case(V2, 11, Some(0), Some(16), None, 8, None, "11.0.16+8")]
    #[case(V2, 11, Some(0), Some(15), None, 10, None, "11.0.15+10")]
    #[case(V2, 11, Some(0), Some(14), Some(1), 1, None, "11.0.14.1+1")]
    #[case(V2, 11, Some(0), Some(14), None, 9, None, "11.0.14+9")]
    #[case(V2, 11, Some(0), Some(13), None, 8, None, "11.0.13+8")]
    #[case(V2, 11, Some(0), Some(12), None, 7, None, "11.0.12+7")]
    #[case(V1, 1, Some(8), Some(0), None, 372, Some("b07"), "1.8.0_372-b07")]
    #[case(V1, 1, Some(8), Some(0), None, 362, Some("b09"), "1.8.0_362-b09")]
    #[case(V1, 1, Some(8), Some(0), None, 352, Some("b08"), "1.8.0_352-b08")]
    #[case(V1, 1, Some(8), Some(0), None, 345, Some("b01"), "1.8.0_345-b01")]
    #[case(V1, 1, Some(8), Some(0), None, 342, Some("b07"), "1.8.0_342-b07")]
    #[case(V1, 1, Some(8), Some(0), None, 332, Some("b09"), "1.8.0_332-b09")]
    #[case(V1, 1, Some(8), Some(0), None, 322, Some("b06"), "1.8.0_322-b06")]
    #[case(V1, 1, Some(8), Some(0), None, 312, Some("b07"), "1.8.0_312-b07")]
    #[case(V1, 1, Some(8), Some(0), None, 302, Some("b08"), "1.8.0_302-b08")]
    fn basics(
        #[case] expected_kind: OpenJdkVersionKind,
        #[case] expected_major: u32,
        #[case] expected_minor: Option<u32>,
        #[case] expected_patch: Option<u32>,
        #[case] expected_build: Option<u32>,
        #[case] expected_qualifier1: u32,
        #[case] expected_qualifier2: Option<&str>,
        #[case] input: &str,
    ) -> Result<()> {
        let version = input.parse::<OpenJdkVersion>()?;
        assert_eq!(expected_kind, version.kind);
        assert_eq!(expected_major, version.major);
        assert_eq!(expected_minor, version.minor);
        assert_eq!(expected_patch, version.patch);
        assert_eq!(expected_build, version.build);
        assert_eq!(expected_qualifier1, version.qualifier1);
        assert_eq!(expected_qualifier2.map(String::from), version.qualifier2);
        assert_eq!(input, version.to_string());
        Ok(())
    }

    #[test]
    fn order() -> Result<()> {
        let mut versions = [
            "20.0.1+9",
            "20+36",
            "19.0.2+7",
            "19.0.1+10",
            "19+36",
            "18.0.2.1+1",
            "18.0.2+9",
            "18.0.1+10",
            "17.0.7+7",
            "17.0.6+10",
            "17.0.5+8",
            "17.0.4.1+1",
            "17.0.4+8",
            "17.0.3+7",
            "17.0.2+8",
            "17.0.1+12",
            "11.0.19+7",
            "11.0.18+10",
            "11.0.17+8",
            "11.0.16.1+1",
            "11.0.16+8",
            "11.0.15+10",
            "11.0.14.1+1",
            "11.0.14+9",
            "11.0.13+8",
            "11.0.12+7",
            "1.8.0_372-b07",
            "1.8.0_362-b09",
            "1.8.0_352-b08",
            "1.8.0_345-b01",
            "1.8.0_342-b07",
            "1.8.0_332-b09",
            "1.8.0_322-b06",
            "1.8.0_312-b07",
            "1.8.0_302-b08",
        ]
        .iter()
        .map(|s| s.parse::<OpenJdkVersion>())
        .collect::<OpenJdkVersionParseResult<Vec<_>>>()?;
        versions.sort();

        let versions_in_order = vec![
            "1.8.0_302-b08",
            "1.8.0_312-b07",
            "1.8.0_322-b06",
            "1.8.0_332-b09",
            "1.8.0_342-b07",
            "1.8.0_345-b01",
            "1.8.0_352-b08",
            "1.8.0_362-b09",
            "1.8.0_372-b07",
            "11.0.12+7",
            "11.0.13+8",
            "11.0.14+9",
            "11.0.14.1+1",
            "11.0.15+10",
            "11.0.16+8",
            "11.0.16.1+1",
            "11.0.17+8",
            "11.0.18+10",
            "11.0.19+7",
            "17.0.1+12",
            "17.0.2+8",
            "17.0.3+7",
            "17.0.4+8",
            "17.0.4.1+1",
            "17.0.5+8",
            "17.0.6+10",
            "17.0.7+7",
            "18.0.1+10",
            "18.0.2+9",
            "18.0.2.1+1",
            "19+36",
            "19.0.1+10",
            "19.0.2+7",
            "20+36",
            "20.0.1+9",
        ]
        .iter()
        .map(|s| s.parse::<OpenJdkVersion>())
        .collect::<OpenJdkVersionParseResult<Vec<_>>>()?;

        assert_eq!(versions_in_order, versions);
        Ok(())
    }
}
