#[derive(Debug, PartialEq)]
pub enum OS {
    Darwin,
    Linux,
    Windows,
}

impl OS {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "darwin" => Self::Darwin,
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            _ => return None,
        })
    }
}
