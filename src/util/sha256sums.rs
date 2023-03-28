use super::osstring_to_string;
use crate::config::Config;
use crate::error::{fatal, Result};
use crate::object_model::Tag;
use hex::decode;
use include_dir::{include_dir, Dir};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{read, read_dir};

static SHA256SUMS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sha256sums");

pub fn dump_sha256sums(config: &Config, tag: &Tag) -> Result<()> {
    let file_name = format!("{}.sha256sums", tag.as_str());
    let f = SHA256SUMS_DIR
        .get_file(file_name)
        .ok_or(fatal("Resource not found"))?;
    let s = f
        .contents_utf8()
        .ok_or(fatal("Resource could not be decoded at UTF-8"))?;

    let mut map = HashMap::new();
    for line in s.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let checksum = parts[0];
        let asset_name = parts[1];
        _ = map.insert(asset_name, checksum);
    }

    for e in read_dir(&config.assets_dir)? {
        let e = e?;
        let file_name = osstring_to_string(e.file_name())?;
        if let Some(expected_hash_str) = map.get(file_name.as_str()) {
            let expected_hash = decode(expected_hash_str)?;
            let mut hasher = Sha256::new();
            hasher.update(read(e.path())?);
            let hash = hasher.finalize().to_vec();
            let result = expected_hash == hash;
            println!("{}: {}", e.path().display(), result);
        }
    }

    Ok(())
}
