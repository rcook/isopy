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
use crate::python_version::PythonVersion;
use crate::serialization::ProjectConfig;
use crate::tag::Tag;
use isopy_lib::{other_error as isopy_lib_other_error, Descriptor, IsopyLibError, IsopyLibResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct PythonDescriptor {
    pub version: PythonVersion,
    pub tag: Option<Tag>,
}

impl FromStr for PythonDescriptor {
    type Err = IsopyLibError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s.split_once(':') {
            Some((prefix, suffix)) => prefix
                .parse::<PythonVersion>()
                .map_err(isopy_lib_other_error)
                .and_then(|version| {
                    suffix
                        .parse::<Tag>()
                        .map_err(isopy_lib_other_error)
                        .map(|tag| Self {
                            version,
                            tag: Some(tag),
                        })
                }),
            None => s
                .parse::<PythonVersion>()
                .map_err(isopy_lib_other_error)
                .map(|version| Self { version, tag: None }),
        }
    }
}

impl Display for PythonDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.tag.as_ref() {
            Some(tag) => write!(f, "{}:{tag}", self.version),
            None => write!(f, "{}", self.version),
        }
    }
}

impl<'de> Deserialize<'de> for PythonDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for PythonDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Descriptor for PythonDescriptor {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transform_archive_path(&self, path: &Path, _bin_subdir: &Path) -> PathBuf {
        path.to_path_buf()
    }

    fn get_env_props(&self, _bin_subdir: &Path) -> IsopyLibResult<Value> {
        /*
        serde_json::to_value(EnvConfig {
            dir: bin_subdir.to_path_buf(),
            descriptor: self.clone(),
        })
        .map_err(isopy_lib_other_error)
        */
        todo!();
    }

    fn get_project_props(&self) -> IsopyLibResult<Value> {
        serde_json::to_value(ProjectConfig {
            descriptor: self.clone(),
        })
        .map_err(isopy_lib_other_error)
    }
}

#[cfg(test)]
mod tests {
    use super::PythonDescriptor;
    use crate::python_version::PythonVersion;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let descriptor = "3.11.1:20230702".parse::<PythonDescriptor>()?;
        assert_eq!(PythonVersion::new(3, 11, 1, "3.11.1"), descriptor.version);
        assert_eq!("20230702", descriptor.tag.unwrap().as_str());
        Ok(())
    }

    #[test]
    fn basics_no_tag() -> Result<()> {
        let descriptor = "3.11.2".parse::<PythonDescriptor>()?;
        assert_eq!(PythonVersion::new(3, 11, 2, "3.11.2"), descriptor.version);
        assert!(descriptor.tag.is_none());
        Ok(())
    }
}
