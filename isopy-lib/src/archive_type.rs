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
use crate::extent::Extent;
use crate::package_manager::InstallPackageOptions;
use crate::progress_indicator::{ProgressIndicator, ProgressIndicatorOptionsBuilder};
use anyhow::{Result, bail};
use flate2::read::GzDecoder;
use log::info;
use std::fs::{File, create_dir_all};
use std::io::{Read, copy};
use std::path::{Component, Path, PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tar::Archive;
use zip::ZipArchive;
use zstd::stream::read::Decoder as ZstdDecoder;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum ArchiveType {
    TarGz,
    TarZst,
    Zip,
}

impl ArchiveType {
    #[must_use]
    pub fn strip_suffix(s: &str) -> Option<(Self, &str)> {
        for value in Self::iter() {
            if let Some(prefix) = s.strip_suffix(value.suffix()) {
                return Some((value, prefix));
            }
        }
        None
    }

    #[must_use]
    pub const fn suffix(&self) -> &str {
        match self {
            Self::TarGz => ".tar.gz",
            Self::TarZst => ".tar.zst",
            Self::Zip => ".zip",
        }
    }

    pub async fn unpack(
        &self,
        archive_path: &Path,
        dir: &Path,
        options: &InstallPackageOptions,
    ) -> Result<()> {
        if dir.exists() {
            bail!("Output directory {} already exists", dir.display())
        }

        let progress_indicator = ProgressIndicator::new(
            &ProgressIndicatorOptionsBuilder::default()
                .enabled(options.show_progress)
                .extent(Extent::Unknown)
                .build()?,
        )?;

        // Decompression is CPU-bound and uses synchronous file I/O; run it on
        // a blocking thread so we don't stall other tasks on this runtime.
        let archive_type = *self;
        let archive_path = archive_path.to_path_buf();
        let dir = dir.to_path_buf();
        let progress = progress_indicator.clone();
        tokio::task::spawn_blocking(move || -> Result<()> {
            match archive_type {
                Self::TarGz => {
                    let file = File::open(&archive_path)?;
                    unpack_tar(GzDecoder::new(file), &dir, &progress)?;
                }
                Self::TarZst => {
                    let file = File::open(&archive_path)?;
                    unpack_tar(ZstdDecoder::new(file)?, &dir, &progress)?;
                }
                Self::Zip => unpack_zip(&archive_path, &dir, &progress)?,
            }
            info!("Unpacked package to {}", dir.display());
            Ok(())
        })
        .await??;

        progress_indicator.finish_and_clear();
        Ok(())
    }
}

fn unpack_tar<R: Read>(reader: R, dir: &Path, progress: &ProgressIndicator) -> Result<()> {
    let mut archive = Archive::new(reader);
    archive.set_preserve_permissions(true);
    archive.set_preserve_mtime(true);
    archive.set_overwrite(true);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let Some(stripped) = strip_one_component(&path) else {
            continue;
        };

        let out_path = dir.join(&stripped);
        reject_traversal(dir, &out_path)?;

        progress.set_message(format!("Unpacking {}", stripped.display()));
        if let Some(parent) = out_path.parent() {
            create_dir_all(parent)?;
        }
        entry.unpack(&out_path)?;
    }

    Ok(())
}

fn unpack_zip(archive_path: &Path, dir: &Path, progress: &ProgressIndicator) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let Some(enclosed) = entry.enclosed_name() else {
            continue;
        };
        let Some(stripped) = strip_one_component(enclosed) else {
            continue;
        };

        let out_path = dir.join(&stripped);
        reject_traversal(dir, &out_path)?;

        progress.set_message(format!("Unpacking {}", stripped.display()));

        if entry.is_dir() {
            create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                create_dir_all(parent)?;
            }
            let mut out = File::create(&out_path)?;
            copy(&mut entry, &mut out)?;

            #[cfg(unix)]
            if let Some(mode) = entry.unix_mode() {
                use std::fs::{Permissions, set_permissions};
                use std::os::unix::fs::PermissionsExt;
                set_permissions(&out_path, Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

fn strip_one_component(path: &Path) -> Option<PathBuf> {
    let mut components = path.components();
    components.next()?;
    let rest: PathBuf = components.collect();
    if rest.as_os_str().is_empty() {
        None
    } else {
        Some(rest)
    }
}

fn reject_traversal(root: &Path, candidate: &Path) -> Result<()> {
    for component in candidate.components() {
        if matches!(component, Component::ParentDir) {
            bail!(
                "Refusing to extract path containing '..': {}",
                candidate.display()
            );
        }
    }
    if !candidate.starts_with(root) {
        bail!(
            "Refusing to extract path outside target directory: {}",
            candidate.display()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn strip_one_component_drops_first_segment() {
        assert_eq!(
            Some(PathBuf::from("bin/python")),
            strip_one_component(Path::new("cpython-3.14/bin/python"))
        );
    }

    #[test]
    fn strip_one_component_returns_none_for_top_level() {
        assert_eq!(None, strip_one_component(Path::new("cpython-3.14")));
    }

    #[test]
    fn strip_one_component_returns_none_for_empty() {
        assert_eq!(None, strip_one_component(Path::new("")));
    }

    #[test]
    fn reject_traversal_allows_nested_paths() {
        let root = Path::new("/tmp/root");
        assert!(reject_traversal(root, &root.join("a/b/c")).is_ok());
    }

    #[test]
    fn reject_traversal_blocks_parent_dir_component() {
        let root = Path::new("/tmp/root");
        let candidate = root.join("..").join("escape");
        assert!(reject_traversal(root, &candidate).is_err());
    }

    #[test]
    fn reject_traversal_blocks_path_outside_root() {
        let root = Path::new("/tmp/root");
        assert!(reject_traversal(root, Path::new("/tmp/other/file")).is_err());
    }
}
