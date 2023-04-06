#[derive(Debug, PartialEq)]
pub enum Arch {
    AArch64,
    I686,
    X86_64,
    X86_64V2,
    X86_64V3,
    X86_64V4,
}

impl Arch {
    pub fn parse<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "aarch64" => Self::AArch64,
            "i686" => Self::I686,
            "x86_64" => Self::X86_64,
            "x86_64_v2" => Self::X86_64V2,
            "x86_64_v3" => Self::X86_64V3,
            "x86_64_v4" => Self::X86_64V4,
            _ => return None,
        })
    }
}
