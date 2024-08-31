use crate::archive_group::ArchiveGroup;
use anyhow::Result;
use isopy_api::PackageVersion;
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

static VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^(?<major>\\d+)\\.(?<minor>\\d+)\\.(?<revision>\\d+)(\\+(?<group>.+))?$")
        .expect("Invalid regex")
});

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ArchiveFullVersion {
    pub version: PackageVersion,
    pub group: ArchiveGroup,
}

impl ArchiveFullVersion {
    pub fn from_keywords(keywords: &mut HashSet<String>) -> Result<Self> {
        let version_regex = &*VERSION_REGEX;

        let mut full_version = None;
        let mut version = None;
        let mut group = None;
        let mut keywords_to_remove = Vec::new();

        for keyword in keywords.iter() {
            if let Some(c) = version_regex.captures(keyword) {
                keywords_to_remove.push(keyword.clone());
                let major = c
                    .get(1)
                    .expect("Must capture major")
                    .as_str()
                    .parse()
                    .expect("Must be integer");
                let minor = c
                    .get(2)
                    .expect("Must capture minor")
                    .as_str()
                    .parse()
                    .expect("Must be integer");
                let revision = c
                    .get(3)
                    .expect("Must capture revision")
                    .as_str()
                    .parse()
                    .expect("Must be integer");

                let temp_version = PackageVersion {
                    major,
                    minor,
                    revision,
                };

                if let Some(m) = c.get(5) {
                    let temp_group = m.as_str().parse()?;
                    assert!(full_version.is_none() && version.is_none() && group.is_none());
                    full_version = Some(Self {
                        version: temp_version,
                        group: temp_group,
                    });
                    break;
                } else {
                    assert!(full_version.is_none() && version.is_none());
                    version = Some(temp_version);
                    if group.is_some() {
                        break;
                    }
                }
            } else if let Ok(temp_group) = keyword.parse::<ArchiveGroup>() {
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
