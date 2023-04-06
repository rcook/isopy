#[derive(Debug, PartialEq)]
pub enum Family {
    CPython,
}

impl Family {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "cpython" => Self::CPython,
            _ => return None,
        })
    }
}
