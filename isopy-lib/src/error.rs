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
use std::error::Error as StdError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
pub enum RenderPathFailedReason {
    IsNotAbsolute,
    PrefixMissing,
    DriveMissing,
}

#[derive(Debug, Error)]
pub enum IsopyLibError {
    #[error("invalid URL {0}: {1}")]
    InvalidUrl(String, anyhow::Error),

    #[error("checksum for file {0} is invalid")]
    ChecksumValidationFailed(PathBuf),

    #[error("version {0} not found")]
    VersionNotFound(String),

    #[error("could not render path {0}")]
    RenderPathFailed(PathBuf, RenderPathFailedReason),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub fn other_error<E>(error: E) -> IsopyLibError
where
    E: StdError + Send + Sync + 'static,
{
    IsopyLibError::Other(anyhow::Error::new(error))
}
