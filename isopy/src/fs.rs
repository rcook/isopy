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
use crate::tng::CONFIG_DIR_NAME;
use anyhow::{anyhow, Result};
use joatmon::{FileReadError, HasOtherError, YamlError};
use std::path::{Path, PathBuf};

pub fn existing<T>(result: Result<T>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(e) => {
            if let Some(e0) = e.downcast_ref::<YamlError>() {
                if let Some(e1) = e0.downcast_other_ref::<FileReadError>() {
                    if e1.is_not_found() {
                        return Ok(None);
                    }
                }
            }
            Err(e)
        }
    }
}

pub fn is_executable_file(path: &Path) -> Result<bool> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn inner(path: &Path) -> Result<bool> {
        use crate::constants::EXECUTABLE_MASK;
        use std::fs::metadata;
        use std::os::unix::fs::PermissionsExt;

        let permissions = metadata(path)?.permissions();
        Ok((permissions.mode() & EXECUTABLE_MASK) != 0)
    }

    #[cfg(target_os = "windows")]
    #[allow(clippy::missing_const_for_fn)]
    #[allow(clippy::unnecessary_wraps)]
    fn inner(_path: &Path) -> Result<bool> {
        Ok(true)
    }

    Ok(path.is_file() && inner(path)?)
}

pub fn ensure_file_executable_mode(path: &Path) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn inner(path: &Path) -> Result<()> {
        use crate::constants::EXECUTABLE_MASK;
        use std::fs::{metadata, set_permissions};
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = metadata(path)?.permissions();
        permissions.set_mode(permissions.mode() | EXECUTABLE_MASK);
        set_permissions(path, permissions)?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[allow(clippy::missing_const_for_fn)]
    #[allow(clippy::unnecessary_wraps)]
    fn inner(_path: &Path) -> Result<()> {
        Ok(())
    }

    inner(path)
}

pub(crate) fn default_config_dir() -> Result<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        Ok(dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not determine configuration directory"))?
            .join(CONFIG_DIR_NAME))
    }

    #[cfg(target_os = "macos")]
    {
        Ok(dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?
            .join(".config")
            .join(CONFIG_DIR_NAME))
    }
}
