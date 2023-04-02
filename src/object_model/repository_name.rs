use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REPOSITORY_NAME_REGEX: Regex = Regex::new("^[A-Za-z0-9-_]+$").unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub struct RepositoryName(String);

impl RepositoryName {
    #[allow(unused)]
    pub fn parse(s: &str) -> Option<Self> {
        match REPOSITORY_NAME_REGEX.is_match(s) {
            true => Some(Self(String::from(s))),
            false => None,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RepositoryName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    #[case(Some(RepositoryName(String::from("foo"))), "foo")]
    #[case(Some(RepositoryName(String::from("foo-_"))), "foo-_")]
    fn test_basics(#[case] expected_result: Option<RepositoryName>, #[case] input: String) {
        assert_eq!(expected_result, RepositoryName::parse(&input))
    }
}
