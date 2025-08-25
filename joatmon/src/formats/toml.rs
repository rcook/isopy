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
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use thiserror::Error;
use toml::de::Error as TomlDeError;
use toml_edit::{DocumentMut, TomlError as TomlEditError};

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum TomlErrorKind {
    Syntax,
    Other,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct TomlError(#[from] TomlErrorImpl);

impl TomlError {
    #[allow(unused)]
    #[must_use]
    pub const fn kind(&self) -> TomlErrorKind {
        match self.0 {
            TomlErrorImpl::Syntax { .. } => TomlErrorKind::Syntax,
            _ => TomlErrorKind::Other,
        }
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_syntax(&self) -> bool {
        self.kind() == TomlErrorKind::Syntax
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_other(&self) -> bool {
        self.kind() == TomlErrorKind::Other
    }

    fn other<E>(e: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(TomlErrorImpl::Other(AnyhowError::new(e)))
    }

    fn convert(e: &TomlDeError, path: &Path) -> Self {
        let message = if let Some(s) = e.span() {
            format!(
                "{} at span {}:{} in {}",
                e.message(),
                s.start,
                s.end,
                path.display()
            )
        } else {
            format!("{} in {}", e.message(), path.display())
        };

        Self(TomlErrorImpl::Syntax {
            message,
            path: path.to_path_buf(),
            span: e.span(),
        })
    }

    fn convert_edit(e: &TomlEditError, path: &Path) -> Self {
        let message = if let Some(s) = e.span() {
            format!(
                "{} at span {}:{} in {}",
                e.message(),
                s.start,
                s.end,
                path.display()
            )
        } else {
            format!("{} in {}", e.message(), path.display())
        };

        Self(TomlErrorImpl::Syntax {
            message,
            path: path.to_path_buf(),
            span: e.span(),
        })
    }
}

impl HasOtherError for TomlError {
    fn is_other(&self) -> bool {
        self.is_other()
    }

    fn downcast_other_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let TomlErrorImpl::Other(ref inner) = self.0 {
            inner.downcast_ref::<E>()
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
enum TomlErrorImpl {
    #[error("{message}")]
    Syntax {
        message: String,
        path: PathBuf,
        span: Option<std::ops::Range<usize>>,
    },
    #[error(transparent)]
    Other(AnyhowError),
}

#[allow(unused)]
pub fn read_toml_file<T>(path: &Path) -> StdResult<T, TomlError>
where
    T: DeserializeOwned,
{
    let s = read_text_file(path).map_err(TomlError::other)?;
    let value = toml::from_str::<T>(&s).map_err(|e| TomlError::convert(&e, path))?;
    Ok(value)
}

#[allow(unused)]
pub fn read_toml_file_edit(path: &Path) -> StdResult<DocumentMut, TomlError> {
    let s = read_text_file(path).map_err(TomlError::other)?;
    let doc = s
        .parse::<DocumentMut>()
        .map_err(|e| TomlError::convert_edit(&e, path))?;
    Ok(doc)
}

#[cfg(test)]
mod tests {
    use super::{TomlErrorKind, read_toml_file, read_toml_file_edit};
    use anyhow::Result;
    use std::fs::write;
    use tempdir::TempDir;
    use toml::{Value, toml};

    #[test]
    fn test_read_toml_file_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.toml");
        write(&path, "message = \"hello-world\"")?;

        // Act
        let value = read_toml_file::<toml::Table>(&path)?;

        // Assert
        assert_eq!(toml!(message = "hello-world"), value);
        Ok(())
    }

    #[test]
    fn test_read_toml_file_invalid_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.toml");
        write(&path, "xxx{\"message\": \"hello-world\"}")?;

        // Act
        let e = match read_toml_file::<Value>(&path) {
            Ok(_) => panic!("read_toml_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(TomlErrorKind::Syntax, e.kind());
        assert!(e.is_syntax());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }

    #[test]
    fn test_read_toml_file_edit_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.toml");
        let cargo_toml = r#"[package]
name = "joatmon"
version = "0.0.0"
edition = "2021"

[lib]
name = "joatmon"
path = "src/lib/mod.rs"

[[bin]]
name = "joatmon"
path = "src/bin/main.rs"

[dependencies]
anyhow = "1.0.70"
colored = "2.0.0"
serde = { version = "1.0.160", features = ["derive"] }
serde_json = "1.0.96"
serde_yaml = "0.9.21"
tempdir = "0.3.7"
thiserror = "1.0.40"
toml = "0.7.3"
toml_edit = "0.19.8"
"#;
        write(&path, cargo_toml)?;

        // Act
        let doc = read_toml_file_edit(&path)?;
        let result = doc.to_string();

        // Assert
        assert_eq!(cargo_toml, result);
        Ok(())
    }

    #[test]
    fn test_read_toml_file_edit_invalid_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.toml");
        write(&path, "xxx{\"message\": \"hello-world\"}")?;

        // Act
        let e = match read_toml_file_edit(&path) {
            Ok(_) => panic!("read_toml_file_edit must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(TomlErrorKind::Syntax, e.kind());
        assert!(e.is_syntax());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }
}
