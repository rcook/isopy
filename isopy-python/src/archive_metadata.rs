use crate::archive_type::ArchiveType;
use anyhow::{bail, Error, Result};
use std::collections::HashSet;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct ArchiveMetadata {
    name: String,
    #[allow(unused)]
    archive_type: ArchiveType,
    #[allow(unused)]
    version: String,
    #[allow(unused)]
    tags: HashSet<String>,
}

impl ArchiveMetadata {
    pub fn name(&self) -> &str {
        &self.name
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

        fn get_version(s: &str, tags: &mut HashSet<String>) -> Result<String> {
            let mut version = None;
            for tag in tags.iter() {
                if tag.starts_with("3") {
                    version = Some(tag.clone());
                    break;
                }
            }
            let Some(version) = version else {
                bail!("Archive {s} is not a valid Python archive")
            };

            tags.remove(&version);
            Ok(version.clone())
        }

        let (prefix, archive_type) = parse_archive_type(s)?;

        let mut tags = prefix.split('-').map(str::to_owned).collect::<HashSet<_>>();
        if !tags.remove("cpython") {
            bail!("Archive {s} is not a valid Python archive")
        }

        let version = get_version(s, &mut tags)?;

        Ok(Self {
            name: String::from(s),
            archive_type,
            version,
            tags,
        })
    }
}
