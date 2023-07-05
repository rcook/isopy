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
pub enum Flavour {
    Gnu,
    Msvc,
    Musl,
}

impl FromStr for Flavour {
    type Err = IsopyPythonError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s {
            "gnu" => Ok(Self::Gnu),
            "msvc" => Ok(Self::Msvc),
            "musl" => Ok(Self::Musl),
            _ => Err(IsopyPythonError::UnsupportedFlavour(String::from(s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Flavour;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(Flavour::Gnu, "gnu")]
    #[case(Flavour::Msvc, "msvc")]
    #[case(Flavour::Musl, "musl")]
    fn parse_basics(#[case] expected_flavour: Flavour, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_flavour, input.parse::<Flavour>()?);
        Ok(())
    }

    #[test]
    fn parse_error() {
        assert!("".parse::<Flavour>().is_err());
    }
}
