#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ArchiveGroup {
    OldStyle(String),
    NewStyle(String),
}
