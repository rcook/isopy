use crate::result::{translate_io_error, translate_json_error, translate_yaml_error, Result};
use serde::de::DeserializeOwned;
use std::fs::{create_dir_all, read_to_string, write, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn safe_create_file<P>(path: P, overwrite: bool) -> Result<File>
where
    P: AsRef<Path>,
{
    let mut options = OpenOptions::new();
    options.write(true);
    if overwrite {
        options.create(true);
    } else {
        options.create_new(true);
    }

    let file = options
        .open(&path)
        .map_err(|e| translate_io_error(e, &path))?;
    Ok(file)
}

pub fn safe_write_file<P, C>(path: P, contents: C, overwrite: bool) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let mut dir = PathBuf::new();
    dir.push(&path);
    dir.pop();
    create_dir_all(&dir).map_err(|e| translate_io_error(e, &dir))?;

    if overwrite {
        write(&path, contents).map_err(|e| translate_io_error(e, &path))?;
    } else {
        let mut file = safe_create_file(&path, overwrite)?;
        file.write_all(contents.as_ref())
            .map_err(|e| translate_io_error(e, &path))?;
    }
    Ok(())
}

pub fn read_text_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    Ok(read_to_string(&path).map_err(|e| translate_io_error(e, &path))?)
}

pub fn read_json_file<T, P>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let json = read_text_file(&path)?;
    read_json_helper(&json, &path)
}

pub fn read_yaml_file<T, P>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let yaml = read_text_file(&path)?;
    read_yaml_helper(&yaml, &path)
}

fn read_json_helper<T, P>(json: &str, path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(serde_json::from_str::<T>(&json).map_err(|e| translate_json_error(e, &path))?)
}

fn read_yaml_helper<T, P>(json: &str, path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(serde_yaml::from_str::<T>(&json).map_err(|e| translate_yaml_error(e, &path))?)
}

#[cfg(test)]
mod tests {
    use super::{read_yaml_file, read_yaml_helper};
    use crate::result::{Error, ErrorInfo, Result};
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn test_file_does_not_exist() -> Result<()> {
        let temp_dir = TempDir::new("isopy-test")?;
        let does_not_exist_path = temp_dir.path().join("does-not-exist");

        let result = read_yaml_file::<i32, _>(&does_not_exist_path);

        let error = result.expect_err("must be an error");
        let (message, path) = match error {
            Error::Reportable { message, info } => match info {
                ErrorInfo::FileNotFound { path, .. } => (message, path),
                _ => panic!(),
            },
            _ => panic!(),
        };
        assert!(message.contains(does_not_exist_path.to_str().expect("must succeed")));
        assert!(path == does_not_exist_path);

        Ok(())
    }

    #[test]
    fn test_empty() -> Result<()> {
        let p = PathBuf::from("/path/to/file");

        let result = read_yaml_helper::<String, _>("", &p);

        let error = result.expect_err("must be an error");
        let (message, path) = match error {
            Error::Reportable { message, info } => match info {
                ErrorInfo::Yaml { path, .. } => (message, path),
                _ => panic!(),
            },
            _ => panic!(),
        };
        assert!(message.contains(p.to_str().expect("must succeed")));
        assert!(path == p);

        Ok(())
    }
}
