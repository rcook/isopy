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
use serde_yaml::{Error as SerdeYamlError, Location};
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use thiserror::Error;

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum YamlErrorKind {
    Syntax,
    Other,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct YamlError(#[from] YamlErrorImpl);

impl YamlError {
    #[allow(unused)]
    #[must_use]
    pub const fn kind(&self) -> YamlErrorKind {
        match self.0 {
            YamlErrorImpl::Syntax { .. } => YamlErrorKind::Syntax,
            _ => YamlErrorKind::Other,
        }
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_syntax(&self) -> bool {
        self.kind() == YamlErrorKind::Syntax
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_other(&self) -> bool {
        self.kind() == YamlErrorKind::Other
    }

    fn other<E>(e: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(YamlErrorImpl::Other(AnyhowError::new(e)))
    }

    fn convert(e: &SerdeYamlError, path: &Path) -> Self {
        Self(YamlErrorImpl::Syntax {
            message: e.to_string(),
            location: e.location(),
            path: path.to_path_buf(),
        })
    }
}

impl HasOtherError for YamlError {
    fn is_other(&self) -> bool {
        self.is_other()
    }

    fn downcast_other_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let YamlErrorImpl::Other(ref inner) = self.0 {
            inner.downcast_ref::<E>()
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
enum YamlErrorImpl {
    #[error("{message} in {path}")]
    Syntax {
        message: String,
        location: Option<Location>,
        path: PathBuf,
    },
    #[error(transparent)]
    Other(AnyhowError),
}

#[allow(unused)]
pub fn read_yaml_file<T>(path: &Path) -> StdResult<T, YamlError>
where
    T: DeserializeOwned,
{
    let s = read_text_file(path).map_err(YamlError::other)?;
    let value = serde_yaml::from_str::<T>(&s).map_err(|e| YamlError::convert(&e, path))?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{YamlErrorKind, read_yaml_file};
    use crate::FileReadError;
    use crate::error::HasOtherError;
    use anyhow::Result;
    use serde_yaml::Value;
    use std::fs::write;
    use tempdir::TempDir;

    #[test]
    fn test_read_yaml_file_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.yaml");
        write(&path, "{\"message\": \"hello-world\"}")?;

        // Act
        let value = read_yaml_file::<Value>(&path)?;

        // Assert
        assert_eq!(
            serde_yaml::from_str::<Value>("message: hello-world").expect("must succeed"),
            value
        );
        Ok(())
    }

    #[test]
    fn test_read_yaml_file_invalid_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.yaml");
        write(&path, "xxx{\"message\": \"hello-world\"}")?;

        // Act
        let e = match read_yaml_file::<Value>(&path) {
            Ok(_) => panic!("read_yaml_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(YamlErrorKind::Syntax, e.kind());
        assert!(e.is_syntax());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }

    #[test]
    fn test_read_yaml_file_nonexistent_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.yaml");

        // Act
        let e = match read_yaml_file::<Value>(&path) {
            Ok(_) => panic!("read_yaml_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(YamlErrorKind::Other, e.kind());
        assert!(!e.is_syntax());
        assert!(e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        assert!(
            e.downcast_other_ref::<FileReadError>()
                .expect("must be Some")
                .is_not_found()
        );

        Ok(())
    }
}
