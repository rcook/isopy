#[derive(Clone, Debug, PartialEq)]
pub struct Tag(String);

impl Tag {
    pub fn parse<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}