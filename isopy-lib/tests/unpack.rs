// Copyright (c) 2026 Richard Cook
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
use std::fs::{read, read_to_string};
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use flate2::Compression;
use flate2::write::GzEncoder;
use isopy_lib::{ArchiveType, InstallPackageOptionsBuilder};
use tar::{Builder as TarBuilder, Header};
use tempfile::TempDir;
use zip::write::{FileOptions, ZipWriter};

const TOP: &str = "cpython-3.14";

struct Entry<'a> {
    path: &'a str,
    data: &'a [u8],
    mode: u32,
}

fn sample_entries() -> Vec<Entry<'static>> {
    vec![
        Entry {
            path: "bin/python",
            data: b"#!/usr/bin/env python\nprint('hi')\n",
            mode: 0o755,
        },
        Entry {
            path: "lib/site-packages/README",
            data: b"hello world",
            mode: 0o644,
        },
    ]
}

fn build_tar_bytes(entries: &[Entry<'_>]) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    {
        let mut builder = TarBuilder::new(&mut buf);
        for e in entries {
            let full = format!("{TOP}/{}", e.path);
            let mut header = Header::new_gnu();
            header.set_size(e.data.len() as u64);
            header.set_mode(e.mode);
            header.set_cksum();
            builder.append_data(&mut header, &full, e.data)?;
        }
        builder.finish()?;
    }
    Ok(buf)
}

fn build_tar_gz(path: &Path, entries: &[Entry<'_>]) -> Result<()> {
    let tar_bytes = build_tar_bytes(entries)?;
    let file = std::fs::File::create(path)?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(&tar_bytes)?;
    encoder.finish()?;
    Ok(())
}

fn build_tar_zst(path: &Path, entries: &[Entry<'_>]) -> Result<()> {
    let tar_bytes = build_tar_bytes(entries)?;
    let file = std::fs::File::create(path)?;
    let mut encoder = zstd::stream::write::Encoder::new(file, 0)?;
    encoder.write_all(&tar_bytes)?;
    encoder.finish()?;
    Ok(())
}

fn build_zip(path: &Path, entries: &[Entry<'_>]) -> Result<()> {
    let file = std::fs::File::create(path)?;
    let mut writer = ZipWriter::new(file);
    for e in entries {
        let full = format!("{TOP}/{}", e.path);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(e.mode);
        writer.start_file(full, options)?;
        writer.write_all(e.data)?;
    }
    writer.finish()?;
    Ok(())
}

fn assert_entries_extracted(dir: &Path, entries: &[Entry<'_>]) -> Result<()> {
    // The unpacker strips the top-level component, so entries land
    // directly under `dir`.
    assert!(
        !dir.join(TOP).exists(),
        "top-level component should have been stripped"
    );

    for e in entries {
        let out = dir.join(e.path);
        assert!(out.is_file(), "expected file {} to exist", out.display());
        let got = read(&out)?;
        assert_eq!(e.data, got.as_slice(), "content mismatch for {}", e.path);
    }
    Ok(())
}

#[cfg(unix)]
fn assert_unix_mode(dir: &Path, path: &str, expected: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let meta = std::fs::metadata(dir.join(path))?;
    let actual = meta.permissions().mode() & 0o777;
    assert_eq!(expected, actual, "mode mismatch for {path}");
    Ok(())
}

async fn unpack(archive: &Path, out: &Path, archive_type: ArchiveType) -> Result<()> {
    let options = InstallPackageOptionsBuilder::default()
        .show_progress(false)
        .build()?;
    archive_type.unpack(archive, out, &options).await
}

#[tokio::test]
async fn tar_gz_extracts_and_strips_top() -> Result<()> {
    let entries = sample_entries();
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("pkg.tar.gz");
    build_tar_gz(&archive, &entries)?;

    let out = tmp.path().join("out");
    unpack(&archive, &out, ArchiveType::TarGz).await?;

    assert_entries_extracted(&out, &entries)?;
    assert_eq!(
        "hello world",
        read_to_string(out.join("lib/site-packages/README"))?
    );

    #[cfg(unix)]
    assert_unix_mode(&out, "bin/python", 0o755)?;

    Ok(())
}

#[tokio::test]
async fn tar_zst_extracts_and_strips_top() -> Result<()> {
    let entries = sample_entries();
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("pkg.tar.zst");
    build_tar_zst(&archive, &entries)?;

    let out = tmp.path().join("out");
    unpack(&archive, &out, ArchiveType::TarZst).await?;

    assert_entries_extracted(&out, &entries)?;
    #[cfg(unix)]
    assert_unix_mode(&out, "bin/python", 0o755)?;

    Ok(())
}

#[tokio::test]
async fn zip_extracts_and_strips_top() -> Result<()> {
    let entries = sample_entries();
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("pkg.zip");
    build_zip(&archive, &entries)?;

    let out = tmp.path().join("out");
    unpack(&archive, &out, ArchiveType::Zip).await?;

    assert_entries_extracted(&out, &entries)?;
    #[cfg(unix)]
    assert_unix_mode(&out, "bin/python", 0o755)?;

    Ok(())
}

#[tokio::test]
async fn rejects_existing_output_directory() -> Result<()> {
    let entries = sample_entries();
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("pkg.tar.gz");
    build_tar_gz(&archive, &entries)?;

    let out = tmp.path().join("out");
    std::fs::create_dir(&out)?;

    let err = unpack(&archive, &out, ArchiveType::TarGz)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("already exists"));
    Ok(())
}

#[tokio::test]
async fn rejects_tar_with_parent_dir_traversal() -> Result<()> {
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("evil.tar.gz");

    // `tar::Builder::append_data` refuses to write `..` components, so we
    // construct the header byte-for-byte to produce a malicious archive.
    let payload = b"owned";
    let mut header = Header::new_gnu();
    header.set_size(payload.len() as u64);
    header.set_mode(0o644);
    header.set_entry_type(tar::EntryType::Regular);
    // Write the path directly into the header bytes, bypassing the safety check.
    let path_bytes = b"top/../../escape.txt";
    let name_field = &mut header.as_old_mut().name[..];
    name_field.fill(0);
    name_field[..path_bytes.len()].copy_from_slice(path_bytes);
    header.set_cksum();

    let mut tar_bytes = Vec::new();
    tar_bytes.extend_from_slice(header.as_bytes());
    tar_bytes.extend_from_slice(payload);
    // tar records are 512-byte aligned; pad payload and add two empty end blocks.
    let pad = (512 - (payload.len() % 512)) % 512;
    tar_bytes.extend(std::iter::repeat_n(0u8, pad));
    tar_bytes.extend(std::iter::repeat_n(0u8, 1024));

    let file = std::fs::File::create(&archive)?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(&tar_bytes)?;
    encoder.finish()?;

    let out = tmp.path().join("out");
    let err = unpack(&archive, &out, ArchiveType::TarGz)
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("..") || msg.contains("outside"), "got: {msg}");

    // Nothing should have escaped.
    assert!(
        !tmp.path().join("escape.txt").exists(),
        "traversal succeeded — file escaped target dir"
    );

    Ok(())
}

#[tokio::test]
async fn rejects_zip_with_parent_dir_traversal() -> Result<()> {
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("evil.zip");

    let file = std::fs::File::create(&archive)?;
    let mut writer = ZipWriter::new(file);
    // Raw write of a traversal path; zip's enclosed_name() typically filters
    // most cases, but we exercise the belt-and-braces reject_traversal check.
    writer.start_file(
        "top/../../escape.txt",
        FileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )?;
    writer.write_all(b"owned")?;
    writer.finish()?;

    let out = tmp.path().join("out");
    let _ = unpack(&archive, &out, ArchiveType::Zip).await;
    // Some zip writers sanitize the path; the important invariant is that
    // nothing ended up outside the target directory.
    assert!(
        !tmp.path().join("escape.txt").exists(),
        "traversal succeeded — file escaped target dir"
    );

    Ok(())
}

#[tokio::test]
async fn tar_gz_creates_nested_directories() -> Result<()> {
    let entries = vec![Entry {
        path: "a/b/c/d/deep.txt",
        data: b"deep",
        mode: 0o644,
    }];
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("pkg.tar.gz");
    build_tar_gz(&archive, &entries)?;

    let out = tmp.path().join("out");
    unpack(&archive, &out, ArchiveType::TarGz).await?;

    assert!(out.join("a/b/c/d").is_dir());
    assert_eq!("deep", read_to_string(out.join("a/b/c/d/deep.txt"))?);
    Ok(())
}

#[tokio::test]
async fn zip_creates_nested_directories() -> Result<()> {
    let entries = vec![Entry {
        path: "a/b/c/d/deep.txt",
        data: b"deep",
        mode: 0o644,
    }];
    let tmp = TempDir::with_prefix("isopy-lib-unpack-")?;
    let archive = tmp.path().join("pkg.zip");
    build_zip(&archive, &entries)?;

    let out = tmp.path().join("out");
    unpack(&archive, &out, ArchiveType::Zip).await?;

    assert!(out.join("a/b/c/d").is_dir());
    assert_eq!("deep", read_to_string(out.join("a/b/c/d/deep.txt"))?);
    Ok(())
}
