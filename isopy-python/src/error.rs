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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IsopyPythonError {
    #[error("Invalid version \"{0}\"")]
    InvalidVersion(String),

    #[error("Invalid repository name \"{0}\"")]
    InvalidRepositoryName(String),

    #[error("Parse error \"{0}\"")]
    ParseError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<strum::ParseError> for IsopyPythonError {
    fn from(value: strum::ParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}

pub fn other_error<E>(error: E) -> IsopyPythonError
where
    E: StdError + Send + Sync + 'static,
{
    IsopyPythonError::Other(anyhow::Error::new(error))
}
