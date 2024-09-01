use anyhow::{bail, Error, Result};
use regex::Regex;
use std::{cmp::Ordering, str::FromStr, sync::LazyLock};

static NEW_STYLE_GROUP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\\d{8}$").expect("Invalid regex"));

static OLD_STYLE_GROUP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\\d{8}T\\d{4}$").expect("Invalid regex"));

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ArchiveGroupInner {
    OldStyle(String),
    NewStyle(String),
}

impl Ord for ArchiveGroupInner {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::OldStyle(a), Self::OldStyle(b)) => a.cmp(b),
            (Self::NewStyle(a), Self::NewStyle(b)) => a.cmp(b),
            (Self::NewStyle(_), Self::OldStyle(_)) => Ordering::Greater,
            (Self::OldStyle(_), Self::NewStyle(_)) => Ordering::Less,
        }
    }
}

impl PartialOrd for ArchiveGroupInner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ArchiveGroup {
    inner: ArchiveGroupInner,
}

impl FromStr for ArchiveGroup {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if NEW_STYLE_GROUP_REGEX.is_match(s) {
            Ok(Self {
                inner: ArchiveGroupInner::NewStyle(String::from(s)),
            })
        } else if OLD_STYLE_GROUP_REGEX.is_match(s) {
            Ok(Self {
                inner: ArchiveGroupInner::OldStyle(String::from(s)),
            })
        } else {
            bail!("Cannot parse {s} as group")
        }
    }
}
