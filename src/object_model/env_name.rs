use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ENV_NAME_REGEX: Regex = Regex::new("^[A-Za-z0-9-_]+$").unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvName(String);

impl EnvName {
    pub fn sanitize(s: &str) -> Self {
        let mut s1 = String::with_capacity(s.len());
        for c in s.chars() {
            if char::is_alphanumeric(c) {
                s1.push(c)
            } else {
                s1.push('-')
            }
        }
        Self::parse(s1).expect("must be a valid environment name")
    }

    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: Into<String>,
    {
        let s1 = s.into();
        match ENV_NAME_REGEX.is_match(&s1) {
            true => Some(Self(s1)),
            false => None,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EnvName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::EnvName;
    use rstest::rstest;

    #[rstest]
    #[case(None, "")]
    #[case(None, " ")]
    #[case(None, " foo ")]
    #[case(Some(EnvName(String::from("foo"))), "foo")]
    #[case(Some(EnvName(String::from("foo-_"))), "foo-_")]
    fn test_basics(#[case] expected_result: Option<EnvName>, #[case] input: String) {
        assert_eq!(expected_result, EnvName::parse(&input))
    }
}
