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
pub enum Subflavour {
    Debug,
    NoOpt,
    PgoLto,
    Pgo,
    Lto,
    Shared,
    Static,
}

impl FromStr for Subflavour {
    type Err = IsopyPythonError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s {
            "debug" => Ok(Self::Debug),
            "noopt" => Ok(Self::NoOpt),
            "pgo+lto" => Ok(Self::PgoLto),
            "pgo" => Ok(Self::Pgo),
            "lto" => Ok(Self::Lto),
            "shared" => Ok(Self::Shared),
            "static" => Ok(Self::Static),
            _ => Err(IsopyPythonError::UnsupportedSubflavour(String::from(s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Subflavour;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(Subflavour::Debug, "debug")]
    #[case(Subflavour::NoOpt, "noopt")]
    #[case(Subflavour::PgoLto, "pgo+lto")]
    #[case(Subflavour::Pgo, "pgo")]
    #[case(Subflavour::Lto, "lto")]
    #[case(Subflavour::Shared, "shared")]
    #[case(Subflavour::Static, "static")]
    fn parse_basics(#[case] expected_subflavour: Subflavour, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_subflavour, input.parse::<Subflavour>()?);
        Ok(())
    }
}
