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
use crate::python_package::PythonPackage;
use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};
use isopy_lib::Checksum;
use std::collections::HashMap;

const SHA256SUMS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/sha256sums");

pub(crate) fn get_checksum(package: &PythonPackage) -> Result<Checksum> {
    fn parse_checksums(content: &str) -> HashMap<&str, &str> {
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

    let release_group_str = package.metadata().index_version().release_group().as_str();
    let file_name = format!("{release_group_str}.sha256sums");
    let file = SHA256SUMS_DIR
        .get_file(&file_name)
        .ok_or_else(|| anyhow!("Resource file {} not found", file_name))?;
    let checksum_strs = parse_checksums(
        file.contents_utf8()
            .ok_or_else(|| anyhow!("Resource file {} could not be decoded as UTF-8", file_name))?,
    );
    let package_name = package.metadata().name();
    let checksum_str = checksum_strs
        .get(package_name)
        .ok_or_else(|| anyhow!("No checksum found for archive {package_name}"))?;
    let checksum = checksum_str.parse()?;
    Ok(checksum)
}
