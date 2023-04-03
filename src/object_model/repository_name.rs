use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REPOSITORY_NAME_REGEX: Regex = Regex::new("^[A-Za-z0-9-_]+$").unwrap();
}

const DEFAULT_REPOSITORY_NAME: &'static str = "default";

#[derive(Clone, Debug, PartialEq)]
pub enum RepositoryName {
    Default,
    Named(String),
}

impl RepositoryName {
    pub fn parse(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case(DEFAULT_REPOSITORY_NAME) {
            Some(Self::Default)
        } else {
            match REPOSITORY_NAME_REGEX.is_match(s) {
                true => Some(Self::Named(String::from(s))),
                false => None,
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => DEFAULT_REPOSITORY_NAME,
            Self::Named(s) => &s,
        }
    }
}

impl std::fmt::Display for RepositoryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryName;
    use rstest::rstest;

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
