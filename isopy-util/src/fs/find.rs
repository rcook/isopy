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
use std::path::{Path, PathBuf};

#[allow(unused)]
#[must_use]
pub fn find_sentinel_dir(
    sentinel_name: &Path,
    start_dir: &Path,
    limit: Option<i32>,
) -> Option<PathBuf> {
    let mut dir = start_dir;
    let mut count = limit.unwrap_or(30);
    loop {
        if count == 0 {
            return None;
        }

        let sentinel_dir_path = dir.join(sentinel_name);
        if sentinel_dir_path.is_dir() {
            return Some(sentinel_dir_path);
        }

        match dir.parent() {
            Some(d) => dir = d,
            None => return None,
        }

        count -= 1;
    }
}

#[allow(unused)]
#[must_use]
pub fn find_sentinel_file(
    sentinel_name: &Path,
    start_dir: &Path,
    limit: Option<i32>,
) -> Option<PathBuf> {
    let mut dir = start_dir;
    let mut count = limit.unwrap_or(30);
    loop {
        if count == 0 {
            return None;
        }

        let sentinel_file_path = dir.join(sentinel_name);
        if sentinel_file_path.is_file() {
            return Some(sentinel_file_path);
        }

        match dir.parent() {
            Some(d) => dir = d,
            None => return None,
        }

        count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{find_sentinel_dir, find_sentinel_file};
    use anyhow::Result;
    use std::fs::create_dir_all;
    use std::fs::write;
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn test_find_sentinel_dir_found() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let start_dir = temp_dir.path().join("aaa").join("bbb").join("ccc");
        let sentinel_dir_path = temp_dir.path().join("aaa").join("SENTINEL");
        create_dir_all(&start_dir)?;
        create_dir_all(&sentinel_dir_path)?;

        // Act
        let value = find_sentinel_dir(Path::new("SENTINEL"), &start_dir, Some(3));

        // Asset
        assert_eq!(Some(sentinel_dir_path), value);
        Ok(())
    }

    #[test]
    fn test_find_sentinel_dir_not_found() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let start_dir = temp_dir.path().join("aaa").join("bbb").join("ccc");
        create_dir_all(&start_dir)?;

        // Act
        let value = find_sentinel_dir(Path::new("SENTINEL"), &start_dir, Some(3));

        // Asset
        assert!(value.is_none());
        Ok(())
    }

    #[test]
    fn test_find_sentinel_file_found() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let start_dir = temp_dir.path().join("aaa").join("bbb").join("ccc");
        let sentinel_file_path = temp_dir.path().join("aaa").join("SENTINEL");
        create_dir_all(&start_dir)?;
        write(&sentinel_file_path, "CONTENTS")?;

        // Act
        let value = find_sentinel_file(Path::new("SENTINEL"), &start_dir, Some(3));

        // Asset
        assert_eq!(Some(sentinel_file_path), value);
        Ok(())
    }

    #[test]
    fn test_find_sentinel_file_not_found() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("isopy-util-test")?;
        let start_dir = temp_dir.path().join("aaa").join("bbb").join("ccc");
        create_dir_all(&start_dir)?;

        // Act
        let value = find_sentinel_file(Path::new("SENTINEL"), &start_dir, Some(3));

        // Asset
        assert!(value.is_none());
        Ok(())
    }
}
