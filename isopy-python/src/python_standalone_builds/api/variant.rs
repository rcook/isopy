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
pub enum Variant {
    #[strum(serialize = "install_only")]
    InstallOnly,

    #[strum(serialize = "full")]
    Full,
}

#[cfg(test)]
mod tests {
    use super::Variant;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(Variant::InstallOnly, "install_only")]
    #[case(Variant::Full, "full")]
    fn parse_basics(#[case] expected_variant: Variant, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_variant, input.parse::<Variant>()?);
        Ok(())
    }
}
