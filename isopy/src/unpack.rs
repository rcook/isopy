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
use crate::util::set_file_mode;
use anyhow::{anyhow, bail, Result};
use flate2::read::GzDecoder;
use isopy_lib::Descriptor;
use joat_logger::{begin_operation, OpProgress};
use joatmon::open_file;
use joatmon::safe_create_file;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{copy, Read};
use std::path::Path;
use tar::{Archive, Entry};
use zip::read::ZipFile;
use zip::ZipArchive;

pub fn unpack_file(
    descriptor: &dyn Descriptor,
    archive_path: &Path,
    output_dir: &Path,
    bin_subdir: &Path,
) -> Result<()> {
    let file_name_lower = archive_path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| {
            anyhow!(
                "could not get file name from path {}",
                archive_path.display()
            )
        })?
        .to_lowercase();

    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    if file_name_lower.ends_with(".tar.gz") {
        unpack_tar_gz(descriptor, archive_path, output_dir, bin_subdir)?;
    } else if file_name_lower.ends_with(".zip") {
        unpack_zip(descriptor, archive_path, output_dir, bin_subdir)?;
    } else {
        bail!("unsupported archive type {}", archive_path.display());
    }

    Ok(())
}

fn unpack_tar_gz(
    descriptor: &dyn Descriptor,
    archive_path: &Path,
    output_dir: &Path,
    bin_subdir: &Path,
) -> Result<()> {
    fn unpack_entry(entry: &mut Entry<GzDecoder<File>>, output_path: &Path) -> Result<()> {
        let containing_dir = output_path.parent().ok_or_else(|| {
            anyhow!(
                "cannot get parent directory from path {}",
                output_path.display()
            )
        })?;

        create_dir_all(containing_dir)?;
        entry.unpack(output_path)?;

        Ok(())
    }

    // Open once to get number of entries (is this necessary?)
    let file = open_file(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let size = archive.entries()?.count();

    // Open a second time to unpack
    let file = open_file(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let op = begin_operation(Some(size as OpProgress))?;
    op.set_message(&format!("Unpacking {}", archive_path.display()));

    for (idx, mut entry) in archive
        .entries()?
        .filter_map(std::result::Result::ok)
        .enumerate()
    {
        let rel_path = descriptor.transform_archive_path(&entry.path()?, bin_subdir);
        let output_path = output_dir.join(rel_path);
        unpack_entry(&mut entry, &output_path)?;
        op.set_progress(idx as OpProgress);
    }

    drop(op);
    Ok(())
}

fn unpack_zip(
    descriptor: &dyn Descriptor,
    archive_path: &Path,
    output_dir: &Path,
    bin_subdir: &Path,
) -> Result<()> {
    fn unpack_entry_dir(mut entry: ZipFile, output_path: &Path) -> Result<()> {
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;
        assert_eq!(0, buf.len());

        create_dir_all(output_path)?;
        if let Some(mode) = entry.unix_mode() {
            set_file_mode(output_path, mode)?;
        }

        Ok(())
    }

    fn unpack_entry_file(mut entry: ZipFile, output_path: &Path) -> Result<()> {
        let containing_dir = output_path.parent().ok_or_else(|| {
            anyhow!(
                "cannot get parent directory from path {}",
                output_path.display()
            )
        })?;

        create_dir_all(containing_dir)?;
        let mut writer = safe_create_file(output_path, true)?;
        _ = copy(&mut entry, &mut writer)?;
        if let Some(mode) = entry.unix_mode() {
            set_file_mode(output_path, mode)?;
        }

        Ok(())
    }

    let file = open_file(archive_path)?;
    let mut archive = ZipArchive::new(file)?;
    let size = archive.len();

    let op = begin_operation(Some(size as OpProgress))?;
    op.set_message(&format!("Unpacking {}", archive_path.display()));

    for idx in 0..size {
        let entry = archive.by_index(idx)?;
        let rel_path = descriptor.transform_archive_path(Path::new(entry.name()), bin_subdir);
        let output_path = output_dir.join(rel_path);
        if entry.is_dir() {
            unpack_entry_dir(entry, &output_path)?;
        } else if entry.is_file() {
            unpack_entry_file(entry, &output_path)?;
        }

        op.set_progress(idx as OpProgress);
    }

    drop(op);
    Ok(())
}
