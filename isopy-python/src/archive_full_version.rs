use crate::archive_group::ArchiveGroup;
use anyhow::Result;
use isopy_api::PackageVersion;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ArchiveFullVersion {
    pub version: PackageVersion,
    pub group: ArchiveGroup,
}

impl ArchiveFullVersion {
    pub fn from_keywords(keywords: &mut HashSet<String>) -> Result<Self> {
        let mut full_version = None;
        let mut version = None;
        let mut group = None;
        let mut keywords_to_remove = Vec::new();

        for keyword in keywords.iter() {
            if let Some((prefix, suffix)) = keyword.split_once('+') {
                if let Ok(temp_version) = prefix.parse() {
                    if let Ok(temp_group) = suffix.parse() {
                        assert!(full_version.is_none() && version.is_none() && group.is_none());
                        keywords_to_remove.push(keyword.clone());
                        full_version = Some(Self {
                            version: temp_version,
                            group: temp_group,
                        });
                        break;
                    }
                }
            }

            if let Ok(temp_version) = keyword.parse() {
                assert!(full_version.is_none() && version.is_none());
                keywords_to_remove.push(keyword.clone());
                version = Some(temp_version);
                if group.is_some() {
                    break;
                }
            }

            if let Ok(temp_group) = keyword.parse() {
                assert!(full_version.is_none() && group.is_none());
                keywords_to_remove.push(keyword.clone());
                group = Some(temp_group);
                if version.is_some() {
                    break;
                }
            }
        }

        for keyword in keywords_to_remove {
            assert!(keywords.remove(&keyword));
        }

        if let Some(result) = full_version {
            assert!(version.is_none() && group.is_none());
            return Ok(result);
        }

        let version = version.expect("Version must be found");
        let group = group.expect("Group must be found");
        Ok(Self { version, group })
    }
}
