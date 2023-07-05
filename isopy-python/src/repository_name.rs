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
use crate::constants::{DEFAULT_REPOSITORY_NAME, EXAMPLE_REPOSITORY_NAME, REPOSITORY_NAME_REGEX};
use crate::error::IsopyPythonError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum RepositoryName {
    Default,
    Example,
    Named(String),
}

impl RepositoryName {
    #[must_use]
    pub const fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => DEFAULT_REPOSITORY_NAME,
            Self::Example => EXAMPLE_REPOSITORY_NAME,
            Self::Named(s) => s,
        }
    }
}

impl FromStr for RepositoryName {
    type Err = IsopyPythonError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        if s.eq_ignore_ascii_case(DEFAULT_REPOSITORY_NAME) {
            return Ok(Self::Default);
        }

        if s.eq_ignore_ascii_case(EXAMPLE_REPOSITORY_NAME) {
            return Ok(Self::Example);
        }

        if REPOSITORY_NAME_REGEX.is_match(s) {
            return Ok(Self::Named(String::from(s)));
        }

        Err(IsopyPythonError::InvalidRepositoryName(String::from(s)))
    }
}

impl Display for RepositoryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> Deserialize<'de> for RepositoryName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for RepositoryName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryName;
    use crate::constants::REPOSITORY_NAME_REGEX;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("default", RepositoryName::Default)]
    #[case("example", RepositoryName::Example)]
    fn special_repository_names(#[case] expected_name: &str, #[case] input: RepositoryName) {
        let name = input.as_str();
        assert_eq!(expected_name, name);
        assert!(REPOSITORY_NAME_REGEX.is_match(name));
    }

    #[rstest]
    #[case(true, "default")]
    #[case(false, "example")]
    #[case(false, "other")]
    fn is_default(#[case] expected_is_default: bool, #[case] input: &str) -> Result<()> {
        assert_eq!(
            expected_is_default,
            input.parse::<RepositoryName>()?.is_default()
        );
        Ok(())
    }

    #[rstest]
    #[case(RepositoryName::Named(String::from("foo")), "foo")]
    #[case(RepositoryName::Named(String::from("foo-_")), "foo-_")]
    #[case(RepositoryName::Default, "DEFAULT")]
    #[case(RepositoryName::Default, "default")]
    fn parse_basics(#[case] expected_result: RepositoryName, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse::<RepositoryName>()?);
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    #[case(" foo ")]
    fn parse_errors(#[case] input: &str) {
        assert!(input.parse::<RepositoryName>().is_err());
    }
}
