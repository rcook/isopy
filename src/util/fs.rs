use crate::error::{Error, Result};
use std::fs::{create_dir_all, write, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

pub fn safe_write_to_file<P, C>(path: P, contents: C, overwrite: bool) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let mut dir = PathBuf::new();
    dir.push(&path);
    dir.pop();
    create_dir_all(&dir)?;

    if overwrite {
        write(path, contents)?;
    } else {
        let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
        file.write_all(contents.as_ref())?;
    }
    Ok(())
}

pub fn is_already_exists(e: &Error) -> bool {
    match e {
        Error::IO(x) => x.kind() == ErrorKind::AlreadyExists,
        _ => false,
    }
}
