use crate::tng::archive_info::ArchiveInfo;
use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};
use isopy_lib::tng::Checksum;
use std::collections::HashMap;

const SHA256SUMS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sha256sums");

pub(crate) fn get_checksum(archive: &ArchiveInfo) -> Result<Checksum> {
    fn parse_checksums<'a>(content: &'a str) -> HashMap<&'a str, &'a str> {
        content
            .lines()
            .map(|line| {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                let checksum = parts[0];
                let file_name = parts[1];
                (file_name, checksum)
            })
            .collect::<HashMap<_, _>>()
    }

    let group_str = archive.metadata().full_version().group.as_str();
    let file_name = format!("{group_str}.sha256sums");
    let file = SHA256SUMS_DIR
        .get_file(&file_name)
        .ok_or_else(|| anyhow!("Resource file {} not found", file_name))?;
    let checksum_strs = parse_checksums(
        file.contents_utf8()
            .ok_or_else(|| anyhow!("Resource file {} could not be decoded as UTF-8", file_name))?,
    );
    let archive_name = archive.metadata().name();
    let checksum_str = checksum_strs
        .get(archive_name)
        .ok_or_else(|| anyhow!("No checksum found for archive {archive_name}"))?;
    let checksum = checksum_str.parse()?;
    Ok(checksum)
}
