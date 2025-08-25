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
use crate::error::RepoError;
use md5::compute;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LinkId(String);

impl FromStr for LinkId {
    type Err = RepoError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(s.to_lowercase()))
        } else {
            Err(RepoError::invalid_link_id(s))
        }
    }
}

impl TryFrom<&Path> for LinkId {
    type Error = RepoError;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        if !value.is_absolute() {
            return Err(RepoError::could_not_compute_hash(value));
        }

        let s = value
            .to_str()
            .ok_or_else(|| RepoError::could_not_compute_hash(value))?;
        let digest = compute(s);
        format!("{digest:x}").parse::<Self>()
    }
}

impl Display for LinkId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Serialize for LinkId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for LinkId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(String::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::LinkId;
    use anyhow::Result;
    use rstest::rstest;
    use std::path::{Path, PathBuf};

    #[test]
    fn from_path_basics() {
        fn absolute_path() -> PathBuf {
            #[cfg(target_os = "windows")]
            {
                PathBuf::from("C:\\absolute")
            }
            #[cfg(not(target_os = "windows"))]
            {
                PathBuf::from("/absolute")
            }
        }

        assert!(LinkId::try_from(&absolute_path() as &Path).is_ok());
    }

    #[test]
    fn from_path_not_absolute_path() {
        assert!(LinkId::try_from(Path::new("garbage")).is_err());
    }

    #[rstest]
    #[case("abcdef")]
    #[case("ABCDEF")]
    #[case("123456")]
    fn parse_basics(#[case] input: &str) -> Result<()> {
        assert_eq!(input.to_lowercase(), input.parse::<LinkId>()?.to_string());
        Ok(())
    }

    #[rstest]
    #[case("abcdefghijklmnopqrstuvwxyz")]
    #[case("")]
    #[case("  ")]
    #[case("  abcdef  ")]
    fn parse_errors(#[case] input: &str) {
        assert!(input.parse::<LinkId>().is_err());
    }
}
