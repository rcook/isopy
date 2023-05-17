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
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, PartialEq)]
pub enum RepositoryName {
    Default,
    Example,
    Named(String),
}

impl RepositoryName {
    pub fn parse(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case(DEFAULT_REPOSITORY_NAME) {
            return Some(Self::Default);
        }

        if s.eq_ignore_ascii_case(EXAMPLE_REPOSITORY_NAME) {
            return Some(Self::Example);
        }

        if REPOSITORY_NAME_REGEX.is_match(s) {
            return Some(Self::Named(String::from(s)));
        }

        None
    }

    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => DEFAULT_REPOSITORY_NAME,
            Self::Example => EXAMPLE_REPOSITORY_NAME,
            Self::Named(s) => s,
        }
    }
}

impl Display for RepositoryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryName;
    use crate::constants::REPOSITORY_NAME_REGEX;
    use rstest::rstest;

    #[rstest]
    #[case("default", RepositoryName::Default)]
    #[case("example", RepositoryName::Example)]
    fn special_repository_names(#[case] expected_name: &str, #[case] input: RepositoryName) {
        let name = input.as_str();
        assert_eq!(expected_name, name);
        assert!(REPOSITORY_NAME_REGEX.is_match(name))
    }

    #[rstest]
    #[case(true, "default")]
    #[case(false, "example")]
    #[case(false, "other")]
    fn is_default(#[case] expected_is_default: bool, #[case] input: &str) {
        assert_eq!(
            expected_is_default,
            RepositoryName::parse(input)
                .expect("test: must be valid repository name")
                .is_default()
        );
    }

    #[rstest]
    #[case(None, "")]
    #[case(None, " ")]
    #[case(None, " foo ")]
    #[case(Some(RepositoryName::Named(String::from("foo"))), "foo")]
    #[case(Some(RepositoryName::Named(String::from("foo-_"))), "foo-_")]
    #[case(Some(RepositoryName::Default), "DEFAULT")]
    #[case(Some(RepositoryName::Default), "default")]
    fn test_basics(#[case] expected_result: Option<RepositoryName>, #[case] input: String) {
        assert_eq!(expected_result, RepositoryName::parse(&input))
    }
}
