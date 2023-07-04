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
use crate::{python_descriptor::PythonDescriptor, serialization::ProjectConfigRec};
use anyhow::anyhow;
use async_trait::async_trait;
use isopy_lib::{
    Descriptor, DownloadAssetResult, ParseDescriptorError, ParseDescriptorResult, Product,
    ReadProjectConfigFileError, ReadProjectConfigFileResult,
};
use joatmon::read_yaml_file;
use std::path::{Path, PathBuf};

const NAME: &str = "Python";

pub const PYTHON_PROJECT_CONFIG_FILE_NAME: &str = ".python-version.yaml";

pub struct Python;

impl Default for Python {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Product for Python {
    fn name(&self) -> &str {
        NAME
    }

    fn project_config_file_name(&self) -> &Path {
        Path::new(PYTHON_PROJECT_CONFIG_FILE_NAME)
    }

    fn read_project_config_file(
        &self,
        path: &Path,
    ) -> ReadProjectConfigFileResult<Box<dyn Descriptor>> {
        let project_config_rec = read_yaml_file::<ProjectConfigRec>(path)
            .map_err(|e| ReadProjectConfigFileError::Other(anyhow!(e)))?;

        Ok(Box::new(PythonDescriptor {
            version: project_config_rec.version,
            tag: project_config_rec.tag,
        }))
    }

    fn parse_descriptor(&self, s: &str) -> ParseDescriptorResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<PythonDescriptor>()
                .map_err(|e| ParseDescriptorError::Other(anyhow!(e)))?,
        ))
    }

    async fn download_asset(
        &self,
        _descriptor: &dyn Descriptor,
        _shared_dir: &Path,
    ) -> DownloadAssetResult<PathBuf> {
        todo!();
    }
}
