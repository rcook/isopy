use crate::tng::archive_full_version::ArchiveFullVersion;
use crate::tng::archive_type::ArchiveType;
use anyhow::{bail, Error, Result};
use std::collections::HashSet;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub(crate) struct ArchiveMetadata {
    name: String,
    archive_type: ArchiveType,
    full_version: ArchiveFullVersion,
    keywords: HashSet<String>,
}

impl ArchiveMetadata {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    #[allow(unused)]
    pub(crate) fn archive_type(&self) -> &ArchiveType {
        &self.archive_type
    }

    pub(crate) fn full_version(&self) -> &ArchiveFullVersion {
        &self.full_version
    }

    pub(crate) fn keywords(&self) -> &HashSet<String> {
        &self.keywords
    }
}

impl FromStr for ArchiveMetadata {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn parse_archive_type(s: &str) -> Result<(&str, ArchiveType)> {
            for archive_type in ArchiveType::iter() {
                if let Some(prefix) = s.strip_suffix(archive_type.suffix()) {
                    return Ok((prefix, archive_type));
                }
            }
            bail!("Archive {s} is not a valid Python archive")
        }

        let name = String::from(s);

        let (prefix, archive_type) = parse_archive_type(s)?;

        let mut keywords = prefix.split('-').map(str::to_owned).collect::<HashSet<_>>();
        if !keywords.remove("cpython") {
            bail!("Archive {s} is not a valid Python archive")
        }

        let full_version = ArchiveFullVersion::from_keywords(&mut keywords)?;

        Ok(Self {
            name,
            archive_type,
            full_version,
            keywords,
        })
    }
}
