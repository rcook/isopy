#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackageVersion {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
}
