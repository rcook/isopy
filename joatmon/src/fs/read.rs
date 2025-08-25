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
use anyhow::Error as AnyhowError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::fs::{read, read_to_string, File};
use std::io::Error as IOError;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use thiserror::Error;

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FileReadErrorKind {
    IsADirectory,
    NotFound,
    Other,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct FileReadError(#[from] FileReadErrorImpl);

impl FileReadError {
    #[allow(unused)]
    #[must_use]
    pub const fn kind(&self) -> FileReadErrorKind {
        match self.0 {
            FileReadErrorImpl::IsADirectory(_) => FileReadErrorKind::IsADirectory,
            FileReadErrorImpl::NotFound(_) => FileReadErrorKind::NotFound,
            _ => FileReadErrorKind::Other,
        }
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_is_a_directory(&self) -> bool {
        self.kind() == FileReadErrorKind::IsADirectory
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_not_found(&self) -> bool {
        self.kind() == FileReadErrorKind::NotFound
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_other(&self) -> bool {
        self.kind() == FileReadErrorKind::Other
    }

    fn other<E>(e: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(FileReadErrorImpl::Other(AnyhowError::new(e)))
    }

    fn convert(e: IOError, path: &Path) -> Self {
        use std::io::ErrorKind::{self, *};

        #[cfg(target_os = "windows")]
        fn is_is_a_directory(kind: ErrorKind, path: &Path) -> bool {
            kind == PermissionDenied && path.is_dir()
        }

        #[cfg(not(target_os = "windows"))]
        fn is_is_a_directory(kind: ErrorKind, _path: &Path) -> bool {
            // io_error_more adds std::io::ErrorKind::IsADirectory etc.
            // https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.IsADirectory
            // For now, we'll match on the debug string for these unstable values
            format!("{kind:?}").as_str() == "IsADirectory"
        }

        let kind = e.kind();

        if is_is_a_directory(kind, path) {
            return Self(FileReadErrorImpl::IsADirectory(path.to_path_buf()));
        }

        if kind == NotFound {
            return Self(FileReadErrorImpl::NotFound(path.to_path_buf()));
        }

        Self::other(e)
    }
}

impl HasOtherError for FileReadError {
    fn is_other(&self) -> bool {
        self.is_other()
    }

    fn downcast_other_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let FileReadErrorImpl::Other(ref inner) = self.0 {
            inner.downcast_ref::<E>()
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
enum FileReadErrorImpl {
    #[error("File system object {0} is a directory not a file")]
    IsADirectory(PathBuf),
    #[error("File {0} not found")]
    NotFound(PathBuf),
    #[error(transparent)]
    Other(AnyhowError),
}

#[allow(unused)]
pub fn read_text_file(path: &Path) -> StdResult<String, FileReadError> {
    read_to_string(path).map_err(|e| FileReadError::convert(e, path))
}

#[allow(unused)]
pub fn open_file(path: &Path) -> StdResult<File, FileReadError> {
    File::open(path).map_err(|e| FileReadError::convert(e, path))
}

#[allow(unused)]
pub fn read_bytes(path: &Path) -> StdResult<Vec<u8>, FileReadError> {
    read(path).map_err(|e| FileReadError::convert(e, path))
}

#[cfg(test)]
mod tests {
    use super::{open_file, read_bytes, read_text_file, FileReadErrorKind};
    use anyhow::Result;
    use std::fs::write;
    use std::io::Read;
    use tempdir::TempDir;

    #[test]
    fn test_read_text_file_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        let value = read_text_file(&path)?;

        // Assert
        assert_eq!("hello-world", value);
        Ok(())
    }

    #[test]
    fn test_read_text_file_is_a_directory_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;

        // Act
        let e = match read_text_file(temp_dir.path()) {
            Ok(_) => panic!("read_text_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(FileReadErrorKind::IsADirectory, e.kind());
        assert!(e.is_is_a_directory());
        assert!(!e.is_not_found());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(temp_dir.path().to_str().expect("must be valid string")));
        Ok(())
    }

    #[test]
    fn test_read_text_file_not_found_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        let e = match read_text_file(&path) {
            Ok(_) => panic!("read_text_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(FileReadErrorKind::NotFound, e.kind());
        assert!(!e.is_is_a_directory());
        assert!(e.is_not_found());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }

    #[test]
    fn test_open_file_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        let mut file = open_file(&path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Assert
        assert_eq!("hello-world", String::from_utf8(buffer)?);
        Ok(())
    }

    #[test]
    fn test_open_file_not_found_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        let e = match open_file(&path) {
            Ok(_) => panic!("open_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(FileReadErrorKind::NotFound, e.kind());
        assert!(e.is_not_found());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }

    #[test]
    fn test_read_bytes_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        let value = read_bytes(&path)?;

        // Assert
        assert_eq!(br"hello-world".to_vec(), value);
        Ok(())
    }

    #[test]
    fn test_read_bytes_not_found_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("joatmon-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        let e = match read_bytes(&path) {
            Ok(_) => panic!("read_bytes must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(FileReadErrorKind::NotFound, e.kind());
        assert!(e.is_not_found());
        assert!(!e.is_other());
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }
}
