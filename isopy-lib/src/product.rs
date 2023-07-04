use crate::EnvInfo;
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
use crate::descriptor::Descriptor;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadProjectConfigFileError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ReadProjectConfigFileResult<T> = StdResult<T, ReadProjectConfigFileError>;

#[derive(Debug, Error)]
pub enum ParseDescriptorError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ParseDescriptorResult<T> = StdResult<T, ParseDescriptorError>;

#[derive(Debug, Error)]
pub enum DownloadAssetError {
    #[error("version {0} not found")]
    VersionNotFound(String),

    #[error("checksum validation failed on {0}")]
    ChecksumValidationFailed(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type DownloadAssetResult<T> = StdResult<T, DownloadAssetError>;

#[derive(Debug, Error)]
pub enum ReadEnvConfigError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ReadEnvConfigResult<T> = StdResult<T, ReadEnvConfigError>;

#[async_trait]
pub trait Product {
    fn name(&self) -> &str;

    fn project_config_file_name(&self) -> &Path;

    fn read_project_config_file(
        &self,
        path: &Path,
    ) -> ReadProjectConfigFileResult<Box<dyn Descriptor>>;

    fn parse_descriptor(&self, s: &str) -> ParseDescriptorResult<Box<dyn Descriptor>>;

    async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        shared_dir: &Path,
    ) -> DownloadAssetResult<PathBuf>;

    fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> ReadEnvConfigResult<EnvInfo>;
}
