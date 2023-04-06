#[derive(Debug, PartialEq)]
pub enum Platform {
    PC,
    Apple,
    Unknown,
}

impl Platform {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "pc" => Self::PC,
            "apple" => Self::Apple,
            "unknown" => Self::Unknown,
            _ => return None,
        })
    }
}
