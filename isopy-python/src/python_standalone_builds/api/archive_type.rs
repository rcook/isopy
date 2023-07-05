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
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ArchiveTypeBaseName {
    pub archive_type: ArchiveType,
    pub base_name: String,
}

impl FromStr for ArchiveTypeBaseName {
    type Err = IsopyPythonError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let s0 = s;
        for (ext, archive_type) in [
            (".tar.gz", ArchiveType::TarGZ),
            (".tar.zst", ArchiveType::TarZST),
        ] {
            if s0.ends_with(ext) {
                let ext_len = ext.len();
                let base_name = &s0[..s0.len() - ext_len];
                return Ok(Self {
                    archive_type,
                    base_name: String::from(base_name),
                });
            }
        }

        Err(IsopyPythonError::UnsupportedArchiveType(String::from(s)))
    }
}

#[derive(Debug, PartialEq)]
pub enum ArchiveType {
    TarGZ,
    TarZST,
}

#[cfg(test)]
mod tests {
    use super::{ArchiveType, ArchiveTypeBaseName};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(ArchiveType::TarGZ, "", ".tar.gz")]
    #[case(ArchiveType::TarGZ, "file", "file.tar.gz")]
    #[case(ArchiveType::TarZST, "", ".tar.zst")]
    #[case(ArchiveType::TarZST, "file", "file.tar.zst")]
    fn parse_basics(
        #[case] expected_archive_type: ArchiveType,
        #[case] expected_base_name: &str,
        #[case] input: &str,
    ) -> Result<()> {
        let expected_result = ArchiveTypeBaseName {
            archive_type: expected_archive_type,
            base_name: String::from(expected_base_name),
        };
        assert_eq!(expected_result, input.parse::<ArchiveTypeBaseName>()?);
        Ok(())
    }
}
