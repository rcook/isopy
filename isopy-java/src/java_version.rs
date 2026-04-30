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
use anyhow::{Error, bail};
use isopy_lib::{Extra, VersionOps, parse_last_part};
use std::any::Any;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct JavaVersion {
    major: u32,
    minor: Option<u32>,
    build: Option<u32>,
    extra: Extra,
    raw: String,
}

impl Display for JavaVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.raw)
    }
}

impl FromStr for JavaVersion {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let raw = String::from(s);

        let Some(rest) = s.strip_prefix("jdk-") else {
            bail!("Invalid Java version {raw}")
        };

        let parts = rest.split('.').collect::<Vec<_>>();
        let (major, minor, build, extra) = match parts.len() {
            1 => {
                let (major, extra) = parse_last_part("Java", &raw, parts[0])?;
                (major, None, None, extra)
            }
            2 => {
                let major = parts[0].parse()?;
                let (minor, extra) = parse_last_part("Java", &raw, parts[1])?;
                (major, Some(minor), None, extra)
            }
            3 => {
                let major = parts[0].parse()?;
                let minor = parts[1].parse()?;
                let (build, extra) = parse_last_part("Java", &raw, parts[2])?;
                (major, Some(minor), Some(build), extra)
            }
            _ => bail!("Invalid Java version {raw}"),
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
    fn as_str(&self) -> Cow<'_, String> {
        Cow::Owned(format!("{self}"))
    }

    fn label(&self) -> Option<Cow<'_, String>> {
        None
    }

    fn box_clone(&self) -> Box<dyn VersionOps> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
