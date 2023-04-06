#[derive(Debug, PartialEq)]
pub enum Variant {
    InstallOnly,
    Full,
}

impl Variant {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "install_only" => Self::InstallOnly,
            "full" => Self::Full,
            _ => return None,
        })
    }
}
