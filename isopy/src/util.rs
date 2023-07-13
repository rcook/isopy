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
use crate::descriptor_info::DescriptorInfo;
use crate::plugin_host::PluginHostRef;
use anyhow::{anyhow, Result};
use colored::Colorize;
use isopy_lib::Package;
use joatmon::{FileReadError, HasOtherError, YamlError};
use std::ffi::OsStr;
use std::fs::metadata;
use std::path::Path;
use std::sync::Arc;

pub fn prettify_descriptor(plugin_host: &PluginHostRef, package: &Package) -> String {
    let descriptor_info = DescriptorInfo {
        plugin_host: Arc::clone(plugin_host),
        descriptor: Arc::clone(&package.descriptor),
    };
    descriptor_info.to_string()
}

pub fn prettify_package(package: &Package, verbose: bool) -> Result<String> {
    let is_file = package.asset_path.is_file();

    let asset_path_display = (if verbose && is_file {
        package.asset_path.to_str()
    } else {
        package.asset_path.file_name().and_then(OsStr::to_str)
    })
    .ok_or_else(|| anyhow!("cannot convert path {}", package.asset_path.display()))?;

    let asset_path_pretty = if is_file {
        asset_path_display.bright_white().bold()
    } else {
        asset_path_display.white()
    };

    let size = if is_file {
        Some(metadata(&package.asset_path)?.len())
    } else {
        None
    };

    Ok(if let Some(size) = size {
        let size_pretty = format!("{}", humanize_size_base_2(size).cyan());
        format!("{asset_path_pretty} ({size_pretty})")
    } else {
        format!("{asset_path_pretty}")
    })
}

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

#[allow(clippy::cast_precision_loss)]
fn humanize_size_base_2(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;
    const TB: u64 = 1024 * 1024 * 1024 * 1024;

    if bytes < KB {
        return format!("{bytes} B");
    }

    if bytes < MB {
        return format!("{:.1} kB", (bytes as f64) / (KB as f64));
    }

    if bytes < GB {
        return format!("{:.1} MB", (bytes as f64) / (MB as f64));
    }

    if bytes < TB {
        return format!("{:.1} GB", (bytes as f64) / (GB as f64));
    }

    format!("{:.1} TB", (bytes as f64) / (TB as f64))
}

#[cfg(test)]
mod tests {
    use super::humanize_size_base_2;
    use rstest::rstest;

    #[rstest]
    #[case("100 B", 100u64)]
    #[case("1023 B", 1023u64)]
    #[case("1.0 kB", 1024u64)]
    #[case("1.0 kB", 1025u64)]
    #[case("2.0 kB", 2048u64)]
    #[case("1.0 MB", 1_048_576_u64)]
    #[case("1.0 GB", 1_073_741_824_u64)]
    #[case("1.0 TB", 1_099_511_627_776_u64)]

    fn test_humanize_size_base_2(#[case] expected_str: &str, #[case] input: u64) {
        assert_eq!(expected_str, &humanize_size_base_2(input));
    }
}

pub fn set_file_mode(path: &Path, mode: u32) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn inner(path: &Path, mode: u32) -> Result<()> {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;
        set_permissions(path, Permissions::from_mode(mode))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[allow(clippy::unnecessary_wraps)]
    const fn inner(_path: &Path, _mode: u32) -> Result<()> {
        Ok(())
    }

    inner(path, mode)
}
