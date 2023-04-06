#[derive(Debug, PartialEq)]
pub enum Subflavour {
    Debug,
    NoOpt,
    PGOLTO,
    PGO,
    LTO,
    Shared,
    Static,
}

impl Subflavour {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "debug" => Self::Debug,
            "noopt" => Self::NoOpt,
            "pgo+lto" => Self::PGOLTO,
            "pgo" => Self::PGO,
            "lto" => Self::LTO,
            "shared" => Self::Shared,
            "static" => Self::Static,
            _ => return None,
        })
    }
}
