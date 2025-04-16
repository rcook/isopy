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
use crate::extra::Extra;
use anyhow::{bail, Error, Result};
use isopy_lib::VersionOps;
use std::any::Any;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct JavaVersion {
    major: u32,
    minor: Option<u32>,
    build: Option<u32>,
    extra: Extra,
    raw: String,
}

impl JavaVersion {
    #[allow(unused)]
    #[must_use]
    pub(crate) const fn major(&self) -> u32 {
        self.major
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn minor(&self) -> Option<u32> {
        self.minor
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn build(&self) -> Option<u32> {
        self.build
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn extra(&self) -> Extra {
        self.extra
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn raw(&self) -> &str {
        self.raw.as_str()
    }
}

impl Display for JavaVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.raw)
    }
}

impl FromStr for JavaVersion {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        fn parse_last_part(raw: &str, s: &str) -> Result<(u32, Extra)> {
            let mut iter = s.chars();
            let mut prefix = String::new();
            let mut rest = String::new();

            for c in iter.by_ref() {
                if !c.is_ascii_digit() {
                    rest.push(c);
                    break;
                }
                prefix.push(c);
            }

            for c in iter {
                rest.push(c);
            }

            let value = prefix.parse()?;

            Ok(if rest.is_empty() {
                (value, Extra::Stable)
            } else if let Some(rest) = rest.strip_prefix("rc") {
                let value1 = rest.parse()?;
                (value, Extra::ReleaseCandidate(value1))
            } else if let Some(rest) = rest.strip_prefix("beta") {
                let value1 = rest.parse()?;
                (value, Extra::Beta(value1))
            } else {
                bail!("Invalid Go version {raw}")
            })
        }

        let raw = String::from(s);

        let Some(rest) = s.strip_prefix("go") else {
            bail!("Invalid Go version {raw}")
        };

        let parts = rest.split('.').collect::<Vec<_>>();
        let (major, minor, build, extra) = match parts.len() {
            1 => {
                let (major, extra) = parse_last_part(&raw, parts[0])?;
                (major, None, None, extra)
            }
            2 => {
                let major = parts[0].parse()?;
                let (minor, extra) = parse_last_part(&raw, parts[1])?;
                (major, Some(minor), None, extra)
            }
            3 => {
                let major = parts[0].parse()?;
                let minor = parts[1].parse()?;
                let (build, extra) = parse_last_part(&raw, parts[2])?;
                (major, Some(minor), Some(build), extra)
            }
            _ => bail!("Invalid Go version {raw}"),
        };

        Ok(Self {
            major,
            minor,
            build,
            extra,
            raw,
        })
    }
}

impl VersionOps for JavaVersion {
    fn as_str(&self) -> Cow<String> {
        Cow::Owned(format!("{self}"))
    }

    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
