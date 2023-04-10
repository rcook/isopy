#[derive(Debug, PartialEq)]
pub enum Subflavour {
    Debug,
    NoOpt,
    PgoLto,
    Pgo,
    Lto,
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
            "pgo+lto" => Self::PgoLto,
            "pgo" => Self::Pgo,
            "lto" => Self::Lto,
            "shared" => Self::Shared,
            "static" => Self::Static,
            _ => return None,
        })
    }
}
