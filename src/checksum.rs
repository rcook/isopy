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
use crate::object_model::Tag;
use anyhow::{anyhow, Result};
use hex::decode;
use include_dir::{include_dir, Dir};
use joatmon::read_bytes;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

static SHA256SUMS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sha256sums");

pub fn validate_sha256_checksum(archive_path: &Path, tag: &Tag) -> Result<bool> {
    let sha256_file_name = format!("{}.sha256sums", tag.as_str());
    let file = SHA256SUMS_DIR
        .get_file(&sha256_file_name)
        .ok_or_else(|| anyhow!("Resource {} not found", sha256_file_name))?;
    let contents = file.contents_utf8().ok_or_else(|| {
        anyhow!(
            "Resource {} could not be decoded as UTF-8",
            sha256_file_name
        )
    })?;

    let mut map = HashMap::new();
    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let checksum = parts[0];
        let asset_name = parts[1];
        _ = map.insert(asset_name, checksum);
    }

    let archive_file_name = archive_path
        .file_name()
        .ok_or_else(|| anyhow!("Could not get file name"))?
        .to_str()
        .ok_or_else(|| anyhow!("Could not get file name"))?;
    match map.get(archive_file_name) {
        None => Ok(false),
        Some(required_hash_str) => verify_sha256_file_checksum(required_hash_str, archive_path),
    }
}

pub fn verify_sha256_file_checksum(required_hash_str: &str, input_path: &Path) -> Result<bool> {
    let required_hash = decode(required_hash_str)?;
    let mut hasher = Sha256::new();
    hasher.update(read_bytes(input_path)?);
    let hash = hasher.finalize().to_vec();
    Ok(required_hash == hash)
}
