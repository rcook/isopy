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
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Arch {
    #[serde(rename = "amd64")]
    Amd64,

    #[serde(rename = "arm6")]
    Arm6,

    #[serde(rename = "arm64")]
    Arm64,

    #[serde(rename = "armv6l")]
    Armv6l,

    #[serde(rename = "bootstrap")]
    Bootstrap,

    #[serde(rename = "386")]
    I386,

    #[serde(rename = "loong64")]
    Loong64,

    #[serde(rename = "mips")]
    Mips,

    #[serde(rename = "mipsle")]
    Mipsle,

    #[serde(rename = "mips64")]
    Mips64,

    #[serde(rename = "mips64le")]
    Mips64le,

    #[serde(rename = "ppc64")]
    Ppc64,

    #[serde(rename = "ppc64le")]
    Ppc64le,

    #[serde(rename = "riscv64")]
    Riscv64,

    #[serde(rename = "s390x")]
    S390x,
}
