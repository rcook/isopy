// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::python_version::PythonVersion;
use anyhow::{Error, anyhow, bail};
use isopy_lib::ArchiveType;
use std::collections::HashSet;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) struct Metadata {
    pub(crate) name: String,
    pub(crate) archive_type: ArchiveType,
    pub(crate) version: PythonVersion,
    pub(crate) tags: HashSet<String>,
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let name = String::from(s);

        let (archive_type, prefix) = ArchiveType::strip_suffix(s)
            .ok_or_else(|| anyhow!("Cannot determine archive type of file name {s}"))?;

        let mut tags = prefix.split('-').map(str::to_owned).collect::<HashSet<_>>();
        if !tags.remove("cpython") {
            bail!("Archive {s} is not a valid Python archive")
        }

        let version = PythonVersion::from_tags(&mut tags)?;
        if let Some(label) = version.label.as_ref() {
            tags.insert(String::from(label.as_str()));
        }

        Ok(Self {
            name,
            archive_type,
            version,
            tags,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::metadata::Metadata;
    use isopy_lib::ArchiveType;
    use rstest::rstest;

    #[rstest]
    #[case(
        "cpython-3.10.17+20250409-aarch64-apple-darwin-install_only.tar.gz",
        3,
        10,
        17,
        "20250409"
    )]
    #[case(
        "cpython-3.14.0rc2+20250414-x86_64-unknown-linux-gnu-install_only.tar.gz",
        3,
        14,
        0,
        "20250414"
    )]
    fn parse_valid(
        #[case] input: &str,
        #[case] major: i32,
        #[case] minor: i32,
        #[case] revision: i32,
        #[case] label: &str,
    ) -> anyhow::Result<()> {
        let m: Metadata = input.parse()?;
        assert_eq!(input, m.name);
        assert!(matches!(m.archive_type, ArchiveType::TarGz));
        assert_eq!(major, m.version.triple.major);
        assert_eq!(minor, m.version.triple.minor);
        assert_eq!(revision, m.version.triple.revision);
        assert_eq!(
            label,
            m.version
                .label
                .as_ref()
                .expect("label must be present")
                .as_str()
        );
        Ok(())
    }

    #[rstest]
    #[case("cpython-3.10.17+20250409-aarch64-apple-darwin-install_only_stripped.tar.gz")]
    #[case("cpython-3.10.17+20250409-aarch64-apple-darwin-debug-full.tar.zst")]
    fn detects_archive_type(#[case] input: &str) -> anyhow::Result<()> {
        let m: Metadata = input.parse()?;
        assert!(matches!(
            m.archive_type,
            ArchiveType::TarGz | ArchiveType::TarZst
        ));
        Ok(())
    }

    #[rstest]
    #[case("cpython-3.10.17+20250409-aarch64-apple-darwin-install_only.unknown")]
    #[case("pypy-3.10.17+20250409-aarch64-apple-darwin-install_only.tar.gz")]
    #[case("not-a-valid-archive")]
    fn parse_invalid(#[case] input: &str) {
        assert!(input.parse::<Metadata>().is_err());
    }
}
