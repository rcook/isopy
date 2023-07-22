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
use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq)]
pub enum Subflavour {
    #[strum(serialize = "debug")]
    Debug,

    #[strum(serialize = "noopt")]
    NoOpt,

    #[strum(serialize = "pgo+lto")]
    PgoLto,

    #[strum(serialize = "pgo")]
    Pgo,

    #[strum(serialize = "lto")]
    Lto,

    #[strum(serialize = "shared")]
    Shared,

    #[strum(serialize = "static")]
    Static,
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
