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
use crate::openjdk_version::OpenJdkVersion;
use crate::serialization::ProjectConfigRec;
use anyhow::anyhow;
use isopy_lib::{
    Descriptor, GetEnvConfigValueError, GetEnvConfigValueResult, GetProjectConfigValueError,
    GetProjectConfigValueResult, ParseDescriptorError, ProjectConfigInfo,
};
use serde::Serialize;
use std::any::Any;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct OpenJdkDescriptor {
    pub version: OpenJdkVersion,
}

impl FromStr for OpenJdkDescriptor {
    type Err = ParseDescriptorError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        s.parse::<OpenJdkVersion>()
            .map_err(|e| ParseDescriptorError::Other(anyhow!(e)))
            .map(|version| Self { version })
    }
}

impl Display for OpenJdkDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.version)
    }
}

impl Descriptor for OpenJdkDescriptor {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transform_archive_path(&self, path: &Path) -> PathBuf {
        let mut i = path.iter();
        _ = i.next();
        Path::new("jdk").join(i)
    }

    fn get_env_config_value(&self) -> GetEnvConfigValueResult<serde_json::Value> {
        #[derive(Serialize)]
        struct EnvRec {
            #[serde(rename = "dir")]
            dir: PathBuf,

            #[serde(rename = "version")]
            version: OpenJdkVersion,
        }

        serde_json::to_value(EnvRec {
            dir: PathBuf::from("openjdk"),
            version: self.version.clone(),
        })
        .map_err(|e| GetEnvConfigValueError::Other(anyhow!(e)))
    }

    fn get_project_config_info(&self) -> GetProjectConfigValueResult<ProjectConfigInfo> {
        let value = serde_json::to_value(ProjectConfigRec {
            version: self.version.clone(),
        })
        .map_err(|e| GetProjectConfigValueError::Other(anyhow!(e)))?;

        Ok(ProjectConfigInfo { value })
    }
}
