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
use crate::constants::ENV_DIR;
use crate::error::{other_error, IsopyOpenJdkError};
use crate::openjdk_version::OpenJdkVersion;
use crate::serialization::{EnvConfigRec, ProjectConfigRec};
use isopy_lib::{other_error as isopy_lib_other_error, Descriptor, IsopyLibResult};
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
    type Err = IsopyOpenJdkError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        s.parse::<OpenJdkVersion>()
            .map_err(other_error)
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
        ENV_DIR.join(i)
    }

    fn get_env_config(&self) -> IsopyLibResult<serde_json::Value> {
        serde_json::to_value(EnvConfigRec {
            dir: ENV_DIR.clone(),
            version: self.version.clone(),
        })
        .map_err(isopy_lib_other_error)
    }

    fn get_project_config(&self) -> IsopyLibResult<serde_json::Value> {
        serde_json::to_value(ProjectConfigRec {
            version: self.version.clone(),
        })
        .map_err(isopy_lib_other_error)
    }
}
