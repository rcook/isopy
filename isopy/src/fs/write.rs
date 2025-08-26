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
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions, create_dir_all, write};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn safe_create_file(path: &Path, overwrite: bool) -> Result<File> {
    ensure_dir(path)?;

    let mut options = OpenOptions::new();
    options.write(true);
    if overwrite {
        options.create(true);
    } else {
        options.create_new(true);
    }

    options
        .open(path)
        .with_context(|| format!("could not create file {path}", path = path.display()))
}

pub fn safe_write_file<C>(path: &Path, contents: C, overwrite: bool) -> Result<()>
where
    C: AsRef<[u8]>,
{
    ensure_dir(path)?;

    if overwrite {
        write(path, contents)
            .with_context(|| format!("cannot overwrite {path}", path = path.display()))?;
    } else {
        let mut file = safe_create_file(path, overwrite)?;
        file.write_all(contents.as_ref())
            .with_context(|| format!("cannot write to file {path}", path = path.display()))?;
    }

    Ok(())
}

fn ensure_dir(file_path: &Path) -> Result<()> {
    let mut dir = PathBuf::new();
    dir.push(file_path);
    dir.pop();
    create_dir_all(&dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fs::{safe_create_file, safe_write_file};
    use anyhow::Result;
    use std::fs::{read_to_string, write};
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_safe_create_file_no_overwrite_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        let mut file = safe_create_file(&path, false)?;
        file.write_all(b"hello-world")?;

        // Assert
        assert_eq!("hello-world", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_create_file_overwrite_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        let mut file = safe_create_file(&path, true)?;
        file.write_all(b"hello-world")?;

        // Assert
        assert_eq!("hello-world", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_create_file_exists_no_overwrite_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        let e = match safe_create_file(&path, false) {
            Ok(_) => panic!("safe_create_file must fail"),
            Err(e) => e,
        };

        // Assert
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        assert_eq!("hello-world", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_create_file_exists_overwrite_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        let mut file = safe_create_file(&path, true)?;
        file.write_all(b"something-else")?;

        // Assert
        assert_eq!("something-else", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_write_file_no_overwrite_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        safe_write_file(&path, "hello-world", false)?;

        // Assert
        assert_eq!("hello-world", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_write_file_overwrite_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");

        // Act
        safe_write_file(&path, "hello-world", true)?;

        // Assert
        assert_eq!("hello-world", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_write_file_exists_no_overwrite_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        let e = match safe_write_file(&path, "something-else", false) {
            Ok(_) => panic!("safe_write_file must fail"),
            Err(e) => e,
        };

        // Assert
        let message = format!("{e}");
        assert!(message.contains(path.to_str().expect("must be valid string")));
        assert_eq!("hello-world", read_to_string(&path)?);
        Ok(())
    }

    #[test]
    fn test_safe_write_file_exists_overwrite_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.txt");
        write(&path, "hello-world")?;

        // Act
        safe_write_file(&path, "something-else", true)?;

        // Assert
        assert_eq!("something-else", read_to_string(&path)?);
        Ok(())
    }
}
