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
use crate::python_plugin::CHECKSUM_BASE_URL;
use anyhow::{anyhow, bail, Result};
use isopy_lib::{Checksum, DownloadAssetOptionsBuilder, PackageManagerContext};
use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) async fn get_checksum(
    ctx: &PackageManagerContext,
    package: &PythonPackage,
    show_progress: bool,
) -> Result<Checksum> {
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

    let Some(release_group) = package.metadata().version().release_group() else {
        bail!("Python package has no release group")
    };

    let release_group_str = release_group.as_str();
    let file_name = format!("{release_group_str}.sha256sums");
    let url = CHECKSUM_BASE_URL.url().join(&file_name)?;

    let options = DownloadAssetOptionsBuilder::default()
        .update(false)
        .show_progress(show_progress)
        .build()?;
    let path = ctx.download_asset(&url, &options).await?;
    let content = read_to_string(path)?;
    let checksum_strs = parse_checksums(&content);

    let package_name = package.metadata().name();
    let checksum_str = checksum_strs
        .get(package_name)
        .ok_or_else(|| anyhow!("No checksum found for archive {package_name}"))?;
    let checksum = checksum_str.parse()?;

    Ok(checksum)
}
