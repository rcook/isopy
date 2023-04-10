#[derive(Debug, PartialEq)]
pub enum Flavour {
    Gnu,
    Msvc,
    Musl,
}

impl Flavour {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "gnu" => Self::Gnu,
            "msvc" => Self::Msvc,
            "musl" => Self::Musl,
            _ => return None,
        })
    }
}
