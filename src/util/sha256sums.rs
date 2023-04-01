use crate::object_model::Tag;
use crate::result::{fatal, Result};
use hex::decode;
use include_dir::{include_dir, Dir};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::read;
use std::path::Path;

static SHA256SUMS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sha256sums");

pub fn validate_sha256_checksum(archive_path: &Path, tag: &Tag) -> Result<bool> {
    let sha256_file_name = format!("{}.sha256sums", tag.as_str());
    let file = SHA256SUMS_DIR
        .get_file(&sha256_file_name)
        .ok_or(fatal(format!("Resource {} not found", sha256_file_name)))?;
    let contents = file.contents_utf8().ok_or(fatal(format!(
        "Resource {} could not be decoded as UTF-8",
        sha256_file_name
    )))?;

    let mut map = HashMap::new();
    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let checksum = parts[0];
        let asset_name = parts[1];
        _ = map.insert(asset_name, checksum);
    }

    let archive_file_name = archive_path
        .file_name()
        .ok_or(fatal("Could not get file name"))?
        .to_str()
        .ok_or(fatal("Could not get file name"))?;
    match map.get(archive_file_name) {
        None => Ok(false),
        Some(expected_hash_str) => {
            let expected_hash = decode(expected_hash_str)?;
            let mut hasher = Sha256::new();
            hasher.update(read(&archive_path)?);
            let hash = hasher.finalize().to_vec();
            Ok(expected_hash == hash)
        }
    }
}
