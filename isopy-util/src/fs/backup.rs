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
use super::paths::{file_name_safe_timestamp, label_file_name};
use chrono::{DateTime, Utc};
use std::fs::{OpenOptions, copy};
use std::io::{ErrorKind as IOErrorKind, Result as IOResult};
use std::path::{Path, PathBuf};

pub fn safe_back_up(path: &Path) -> IOResult<PathBuf> {
    safe_back_up_inner(path, None)
}

fn generate_backup_path(path: &Path, dt: &DateTime<Utc>) -> PathBuf {
    assert!(path.is_file() && path.is_absolute());

    let label = file_name_safe_timestamp(dt);
    label_file_name(path, &label).expect("must succeed")
}

fn safe_back_up_inner(path: &Path, now: Option<DateTime<Utc>>) -> IOResult<PathBuf> {
    assert!(path.is_file() && path.is_absolute());

    let mut backup_path = generate_backup_path(path, &now.unwrap_or_else(Utc::now));
    loop {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&backup_path)
        {
            Ok(_) => break,
            Err(e) if e.kind() == IOErrorKind::AlreadyExists => {
                backup_path = generate_backup_path(path, &Utc::now());
            }
            Err(e) => return Err(e),
        }
    }

    copy(path, &backup_path)?;
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::{TimeZone, Utc};
    use std::fs::{read_dir, read_to_string, write};
    use tempdir::TempDir;

    #[test]
    fn no_conflict() -> Result<()> {
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path = temp_dir.path().join("file.ext");
        write(&path, "CONTENT")?;

        let output_path = safe_back_up_inner(
            &path,
            Some(
                Utc.with_ymd_and_hms(2019, 3, 17, 16, 43, 0)
                    .single()
                    .expect("must be valid"),
            ),
        )?;

        assert_ne!(path, output_path);
        let items = read_dir(temp_dir.path())?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(2, items.len());
        assert_eq!("CONTENT", read_to_string(path)?);
        assert_eq!("CONTENT", read_to_string(output_path)?);

        Ok(())
    }

    #[test]
    fn conflict() -> Result<()> {
        let temp_dir = TempDir::new("isopy-util-test")?;
        let path1 = temp_dir.path().join("file.ext");

        // Deliberately conflicting file name
        let path2 = temp_dir.path().join("file-20190317T164300000Z.ext");

        write(&path1, "CONTENT1")?;
        write(&path2, "CONTENT2")?;

        let output_path = safe_back_up_inner(
            &path1,
            Some(
                Utc.with_ymd_and_hms(2019, 3, 17, 16, 43, 0)
                    .single()
                    .expect("must be valid"),
            ),
        )?;

        assert_ne!(path1, output_path);
        assert_ne!(path2, output_path);
        let items = read_dir(temp_dir.path())?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(3, items.len());
        assert_eq!("CONTENT1", read_to_string(path1)?);
        assert_eq!("CONTENT2", read_to_string(path2)?);
        assert_eq!("CONTENT1", read_to_string(output_path)?);

        Ok(())
    }
}
