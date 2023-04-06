#[derive(Debug, PartialEq)]
pub enum ArchiveType {
    TarGZ,
    TarZST,
}

impl ArchiveType {
    pub fn parse<S>(s: S) -> Option<(Self, String)>
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
