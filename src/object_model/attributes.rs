#[derive(Debug, PartialEq)]
pub enum ArchiveType {
    TarGZ,
    TarZST,
}

impl ArchiveType {
    pub fn from_asset_name<S>(s: S) -> Option<(Self, String)>
    where
        S: AsRef<str>,
    {
        let s0 = s.as_ref();
        for (ext, archive_type) in [(".tar.gz", Self::TarGZ), (".tar.zst", Self::TarZST)] {
            if s0.ends_with(ext) {
                let ext_len = ext.len();
                let base_name = &s0[..s0.len() - ext_len];
                return Some((archive_type, String::from(base_name)));
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Family {
    CPython,
}

impl Family {
    pub fn from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        Some(match s.as_ref() {
            "cpython" => Self::CPython,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    NewStyle(String),
    OldStyle(String),
}

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
    pub fn from_str<S>(s: S) -> Option<Self>
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

#[derive(Debug, PartialEq)]
pub enum Platform {
    PC,
    Apple,
    Unknown,
}

impl Platform {
    pub fn from_str<S>(s: S) -> Option<Self>
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

#[derive(Debug, PartialEq)]
pub enum OS {
    Darwin,
    Linux,
    Windows,
}

impl OS {
    pub fn from_str<S>(s: S) -> Option<Self>
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

#[derive(Debug, PartialEq)]
pub enum Flavour {
    GNU,
    MSVC,
    MUSL,
}

impl Flavour {
    pub fn from_str<S>(s: S) -> Option<Self>
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
    pub fn from_str<S>(s: S) -> Option<Self>
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

#[derive(Debug, PartialEq)]
pub enum Variant {
    InstallOnly,
    Full,
}

impl Variant {
    pub fn from_str<S>(s: S) -> Option<Self>
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
