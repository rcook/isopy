#[derive(Debug, PartialEq)]
pub enum Platform {
    Pc,
    Apple,
    Unknown,
}

impl Platform {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "pc" => Self::Pc,
            "apple" => Self::Apple,
            "unknown" => Self::Unknown,
            _ => return None,
        })
    }
}
