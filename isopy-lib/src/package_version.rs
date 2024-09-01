use anyhow::{bail, Error, Result};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackageVersion {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
}

impl FromStr for PackageVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.splitn(3, '.').collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("Invalid package version {s}")
        }

        let major = parts.get(0).expect("Expected major").parse()?;
        let minor = parts.get(1).expect("Expected minor").parse()?;
        let revision = parts.get(2).expect("Expected revision").parse()?;

        Ok(Self {
            major,
            minor,
            revision,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PackageVersion;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("1.2.3", PackageVersion { major: 1, minor: 2, revision: 3 })]
    fn from_str(#[case] input: &str, #[case] expected: PackageVersion) -> Result<()> {
        assert_eq!(expected, input.parse()?);
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case("1")]
    #[case("1.2")]
    #[case("1.2.3.4")]
    #[case("1.2.three")]
    fn from_str_fail(#[case] input: &str) {
        assert!(input.parse::<PackageVersion>().is_err());
    }
}
