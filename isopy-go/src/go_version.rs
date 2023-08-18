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
use crate::error::{other_error, IsopyGoError};
use crate::extra::Extra;
use crate::result::IsopyGoResult;
use isopy_lib::{Descriptor, IsopyLibResult};
use serde_json::Value;
use std::any::Any;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GoVersion {
    pub major: u32,
    pub minor: u32,
    pub build: Option<u32>,
    pub extra: Extra,
    pub raw: String,
}

impl FromStr for GoVersion {
    type Err = IsopyGoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_last_part(raw: String, s: &str) -> IsopyGoResult<(u32, Extra)> {
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

            let value = prefix.parse::<u32>().map_err(other_error)?;

            Ok(if rest.is_empty() {
                (value, Extra::Stable)
            } else if let Some(rest) = rest.strip_prefix("rc") {
                let value1 = rest.parse::<u32>().map_err(other_error)?;
                (value, Extra::ReleaseCandidate(value1))
            } else if let Some(rest) = rest.strip_prefix("beta") {
                let value1 = rest.parse::<u32>().map_err(other_error)?;
                (value, Extra::Beta(value1))
            } else {
                return Err(IsopyGoError::InvalidVersion(raw));
            })
        }

        let raw = String::from(s);

        let Some(rest) = s.strip_prefix("go") else {
            return Err(IsopyGoError::InvalidVersion(raw));
        };

        let parts = rest.split('.').collect::<Vec<_>>();
        let (major, minor, build, extra) = match parts.len() {
            2 => {
                let major = parts[0].parse::<u32>().map_err(other_error)?;
                let (minor, extra) = parse_last_part(raw.clone(), parts[1])?;
                (major, minor, None, extra)
            }
            3 => {
                let major = parts[0].parse::<u32>().map_err(other_error)?;
                let minor = parts[1].parse::<u32>().map_err(other_error)?;
                let (build, extra) = parse_last_part(raw.clone(), parts[2])?;
                (major, minor, Some(build), extra)
            }
            _ => return Err(IsopyGoError::InvalidVersion(raw)),
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

impl Display for GoVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.raw)
    }
}

impl Descriptor for GoVersion {
    fn as_any(&self) -> &dyn Any {
        todo!();
    }

    fn transform_archive_path(&self, _path: &Path, _bin_subdir: &Path) -> PathBuf {
        todo!();
    }

    fn get_env_props(&self, _bin_subdir: &Path) -> IsopyLibResult<Value> {
        todo!();
    }

    fn get_project_props(&self) -> IsopyLibResult<Value> {
        todo!();
    }
}
