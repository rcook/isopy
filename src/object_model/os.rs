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
use anyhow::{bail, Error};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum OS {
    Darwin,
    Linux,
    Windows,
}

impl FromStr for OS {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "darwin" => Self::Darwin,
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            _ => bail!("unsupported OS \"{}\"", s),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::OS;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(OS::Darwin, "darwin")]
    #[case(OS::Linux, "linux")]
    #[case(OS::Windows, "windows")]
    fn parse_basics(#[case] expected_os: OS, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_os, input.parse::<OS>()?);
        Ok(())
    }
}
