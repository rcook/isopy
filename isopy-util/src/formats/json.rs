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
use crate::error::HasOtherError;
use crate::fs::read_text_file;
use anyhow::Error as AnyhowError;
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeJsonError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use thiserror::Error;

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum JsonErrorKind {
    Data,
    Eof,
    Io,
    Syntax,
    Other,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct JsonError(#[from] JsonErrorImpl);

impl JsonError {
    #[allow(unused)]
    #[must_use]
    pub const fn kind(&self) -> JsonErrorKind {
        match self.0 {
            JsonErrorImpl::Data { .. } => JsonErrorKind::Data,
            JsonErrorImpl::Eof { .. } => JsonErrorKind::Eof,
            JsonErrorImpl::Io { .. } => JsonErrorKind::Io,
            JsonErrorImpl::Syntax { .. } => JsonErrorKind::Syntax,
            _ => JsonErrorKind::Other,
        }
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_data(&self) -> bool {
        self.kind() == JsonErrorKind::Data
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.kind() == JsonErrorKind::Eof
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_io(&self) -> bool {
        self.kind() == JsonErrorKind::Io
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_syntax(&self) -> bool {
        self.kind() == JsonErrorKind::Syntax
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_other(&self) -> bool {
        self.kind() == JsonErrorKind::Other
    }

    fn other<E>(e: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(JsonErrorImpl::Other(AnyhowError::new(e)))
    }

    fn convert(e: &SerdeJsonError, path: &Path) -> Self {
        use serde_json::error::Category::*;

        let message = e.to_string();
        let path = path.to_path_buf();
        Self(match e.classify() {
            Data => JsonErrorImpl::Data { message, path },
            Eof => JsonErrorImpl::Eof { message, path },
            Io => JsonErrorImpl::Io { message, path },
            Syntax => JsonErrorImpl::Syntax { message, path },
        })
    }
}

impl HasOtherError for JsonError {
    fn is_other(&self) -> bool {
        self.is_other()
    }

    fn downcast_other_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let JsonErrorImpl::Other(ref inner) = self.0 {
            inner.downcast_ref::<E>()
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
enum JsonErrorImpl {
    #[error("{message} in {path}")]
    Data { message: String, path: PathBuf },
    #[error("{message} in {path}")]
    Eof { message: String, path: PathBuf },
    #[error("{message} in {path}")]
    Io { message: String, path: PathBuf },
    #[error("{message} in {path}")]
    Syntax { message: String, path: PathBuf },
    #[error(transparent)]
    Other(AnyhowError),
}

#[allow(unused)]
pub fn read_json_file<T>(path: &Path) -> StdResult<T, JsonError>
where
    T: DeserializeOwned,
{
    let s = read_text_file(path).map_err(JsonError::other)?;
    let value = serde_json::from_str::<T>(&s).map_err(|e| JsonError::convert(&e, path))?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{JsonErrorKind, read_json_file};
    use anyhow::Result;
    use serde_json::{Value, json};
    use std::fs::write;
    use tempdir::TempDir;

    #[test]
    fn test_read_json_file_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.json");
        write(&path, "{\"message\": \"hello-world\"}")?;

        // Act
        let value = read_json_file::<Value>(&path)?;

        // Assert
        assert_eq!(json!({"message": "hello-world"}), value);
        Ok(())
    }

    #[test]
    fn test_read_json_file_invalid_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.json");
        write(&path, "xxx{\"message\": \"hello-world\"}")?;

        // Act
        let e = match read_json_file::<Value>(&path) {
            Ok(_) => panic!("read_json_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(JsonErrorKind::Syntax, e.kind());
        assert!(!e.is_data());
        assert!(!e.is_eof());
        assert!(!e.is_io());
        assert!(e.is_syntax());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }
}
