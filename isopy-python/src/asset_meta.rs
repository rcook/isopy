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
use crate::error::IsopyPythonError;
use crate::python_standalone_builds::api::{
    Arch, ArchiveType, ArchiveTypeBaseName, Family, Flavour, Platform, Subflavour, Variant, OS,
};
use crate::python_version::PythonVersion;
use crate::tag::Tag;
use anyhow::{bail, Result};
use log::trace;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct AssetMeta {
    pub archive_type: ArchiveType,
    pub family: Family,
    pub version: PythonVersion,
    pub arch: Arch,
    pub platform: Platform,
    pub os: OS,
    pub flavour: Option<Flavour>,
    pub subflavour0: Option<Subflavour>,
    pub subflavour1: Option<Subflavour>,
    pub variant: Option<Variant>,
    pub parsed_tag: Tag,
}

macro_rules! asset_token {
    ($iter:expr, $type:ty, $asset:expr, $field:expr) => {
        match $iter.next() {
            Some(value) => match value.parse::<$type>() {
                Ok(result) => result,
                Err(_e) => {
                    return Err(IsopyPythonError::InvalidAssetNameToken(
                        String::from($asset),
                        String::from(value),
                    ))
                }
            },
            None => {
                return Err(IsopyPythonError::MissingAssetNameToken(
                    String::from($asset),
                    String::from($field),
                ))
            }
        }
    };
}

impl FromStr for AssetMeta {
    type Err = IsopyPythonError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        fn wrap<'a>(s: &str, label: &str, result: Option<&'a str>) -> Result<&'a str> {
            match result {
                Some(value) => Ok(value),
                None => bail!("missing {} from asset name \"{}\"", label, s),
            }
        }

        fn parse_version_and_tag_opt(s: &str) -> Result<(PythonVersion, Option<Tag>)> {
            let parts = s.split('+').collect::<Vec<_>>();
            let (version_str, tag) = match parts.len() {
                1 => (parts[0], None),
                2 => (parts[0], Some(parts[1].parse::<Tag>()?)),
                _ => bail!("invalid version/tag \"{}\"", s),
            };

            let version = version_str.parse::<PythonVersion>()?;
            Ok((version, tag))
        }

        trace!("Parsing asset name {s}");

        let ArchiveTypeBaseName {
            archive_type,
            base_name,
        } = s.parse::<ArchiveTypeBaseName>()?;

        let mut iter = base_name.split('-');

        let family = asset_token!(iter, Family, s, "family");
        let (version, mut tag_opt) =
            parse_version_and_tag_opt(wrap(s, "version/tag", iter.next())?)?;

        let arch = asset_token!(iter, Arch, s, "architecture");
        let platform = asset_token!(iter, Platform, s, "platform");
        let os = asset_token!(iter, OS, s, "OS");

        let mut flavour_opt = None;
        let mut subflavour0_opt = None;
        let mut subflavour1_opt = None;
        let mut variant_opt = None;

        for token in iter {
            if flavour_opt.is_none() {
                if let Ok(value) = token.parse::<Flavour>() {
                    flavour_opt = Some(value);
                    continue;
                }
            }

            if subflavour0_opt.is_none() {
                if let Ok(value) = token.parse::<Subflavour>() {
                    subflavour0_opt = Some(value);
                    continue;
                }
            }

            if subflavour1_opt.is_none() {
                if let Ok(value) = token.parse::<Subflavour>() {
                    subflavour1_opt = Some(value);
                    continue;
                }
            }

            if variant_opt.is_none() {
                if let Ok(value) = token.parse::<Variant>() {
                    variant_opt = Some(value);
                    continue;
                }
            }

            if tag_opt.is_none() {
                if let Ok(value) = token.parse::<Tag>() {
                    tag_opt = Some(value);
                    continue;
                }
            }

            return Err(IsopyPythonError::InvalidAssetNameToken(
                String::from(s),
                String::from(token),
            ));
        }

        let Some(tag) = tag_opt else {
            return Err(IsopyPythonError::InvalidAssetName(String::from(s)));
        };

        Ok(Self {
            archive_type,
            family,
            version,
            arch,
            platform,
            os,
            flavour: flavour_opt,
            subflavour0: subflavour0_opt,
            subflavour1: subflavour1_opt,
            variant: variant_opt,
            parsed_tag: tag,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::AssetMeta;
    use crate::python_standalone_builds::api::{
        Arch, ArchiveType, Family, Flavour, Platform, Subflavour, Variant, OS,
    };
    use crate::python_version::PythonVersion;
    use crate::tag::Tag;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 9, "3.10.9"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: Some(Variant::Full),
            parsed_tag: "20230116".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 9, "3.10.9"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
            parsed_tag: "20230116".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.9+20230116-aarch64-apple-darwin-install_only.tar.gz"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 2, "3.10.2"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: None,
            parsed_tag: "20220220T1113".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.2-aarch64-apple-darwin-debug-20220220T1113.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarGZ,
            family: Family::CPython,
            version: PythonVersion::new(3, 9, 6, "3.9.6"),
            arch: Arch::X86_64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: None,
            subflavour1: None,
            variant: Some(Variant::InstallOnly),
            parsed_tag: "20210724T1424".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.9.6-x86_64-apple-darwin-install_only-20210724T1424.tar.gz"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 12, "3.10.12"),
            arch: Arch::PPC64LE,
            platform: Platform::Unknown,
            os: OS::Linux,
            flavour: Some(Flavour::Gnu),
            subflavour0: Some(Subflavour::NoOpt),
            subflavour1: None,
            variant: Some(Variant::Full),
            parsed_tag: "20230726".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.12+20230726-ppc64le-unknown-linux-gnu-noopt-full.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 12, "3.10.12"),
            arch: Arch::AArch64,
            platform: Platform::Apple,
            os: OS::Darwin,
            flavour: None,
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: Some(Variant::Full),
            parsed_tag: "20230726".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.12+20230726-aarch64-apple-darwin-debug-full.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 12, "3.10.12"),
            arch: Arch::PPC64LE,
            platform: Platform::Unknown,
            os: OS::Linux,
            flavour: Some(Flavour::Gnu),
            subflavour0: Some(Subflavour::NoOpt),
            subflavour1: None,
            variant: Some(Variant::Full),
            parsed_tag: "20230726".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.12+20230726-ppc64le-unknown-linux-gnu-noopt-full.tar.zst"
    )]
    #[case(
        AssetMeta {
            archive_type: ArchiveType::TarZST,
            family: Family::CPython,
            version: PythonVersion::new(3, 10, 12, "3.10.12"),
            arch: Arch::S390X,
            platform: Platform::Unknown,
            os: OS::Linux,
            flavour: Some(Flavour::Gnu),
            subflavour0: Some(Subflavour::Debug),
            subflavour1: None,
            variant: Some(Variant::Full),
            parsed_tag: "20230726".parse::<Tag>().expect("test: must be valid tag")
        },
        "cpython-3.10.12+20230726-s390x-unknown-linux-gnu-debug-full.tar.zst"
    )]
    fn test_parse(#[case] expected_result: AssetMeta, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse::<AssetMeta>()?);
        Ok(())
    }
}
