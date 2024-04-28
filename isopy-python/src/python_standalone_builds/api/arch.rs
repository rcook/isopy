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
pub enum Arch {
    #[strum(serialize = "aarch64")]
    AArch64,

    #[strum(serialize = "i686")]
    I686,

    #[strum(serialize = "ppc64le")]
    PPC64LE,

    #[strum(serialize = "s390x")]
    S390X,

    #[strum(serialize = "x86_64")]
    X86_64,

    #[strum(serialize = "x86_64_v2")]
    X86_64V2,

    #[strum(serialize = "x86_64_v3")]
    X86_64V3,

    #[strum(serialize = "x86_64_v4")]
    X86_64V4,

    #[strum(serialize = "armv7")]
    ArmV7,
}

#[cfg(test)]
mod tests {
    use super::Arch;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(Arch::AArch64, "aarch64")]
    #[case(Arch::I686, "i686")]
    #[case(Arch::PPC64LE, "ppc64le")]
    #[case(Arch::X86_64, "x86_64")]
    #[case(Arch::X86_64V2, "x86_64_v2")]
    #[case(Arch::X86_64V3, "x86_64_v3")]
    #[case(Arch::X86_64V4, "x86_64_v4")]
    fn parse_basics(#[case] expected_arch: Arch, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_arch, input.parse::<Arch>()?);
        Ok(())
    }

    #[test]
    fn parse_error() {
        assert!("garbage".parse::<Arch>().is_err());
    }
}
