#[derive(Debug, PartialEq)]
pub enum Flavour {
    GNU,
    MSVC,
    MUSL,
}

impl Flavour {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "gnu" => Self::GNU,
            "msvc" => Self::MSVC,
            "musl" => Self::MUSL,
            _ => return None,
        })
    }
}
