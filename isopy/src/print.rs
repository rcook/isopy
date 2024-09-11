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
use crate::dir_info_ext::DirInfoExt;
use crate::fs::existing;
use crate::plugin_host::PluginHostRef;
use crate::registry::Registry;
use crate::serialization::Env;
use crate::table::{table_columns, table_divider, table_line, table_title, Table, TableSettings};
use anyhow::{anyhow, Result};
use colored::{Color, Colorize};
use isopy_lib::Package;
use joat_repo::{DirInfo, Link, Manifest, Repo};
use serde_json::Value;
use std::ffi::OsStr;
use std::fs::metadata;
use std::sync::Arc;

pub(crate) fn print_link(table: &mut Table, link: &Link, idx: Option<usize>) {
    if let Some(i) = idx {
        table_divider!(table, "({i})");
    }

    table_columns!(table, "Project directory", link.project_dir().display());
    table_columns!(table, "Link path", link.link_path().display());
    table_columns!(table, "Created at", link.created_at());
}

pub(crate) fn print_repo(table: &mut Table, repo: &Repo) {
    table_columns!(table, "Lock file", repo.lock_path().display());
    table_columns!(table, "Configuration file", repo.config_path().display());
    table_columns!(table, "Links directory", repo.links_dir().display());
    table_columns!(table, "Container directory", repo.container_dir().display());
    table_columns!(table, "Shared directory", repo.shared_dir().display());
}

pub(crate) fn print_metadir(
    table: &mut Table,
    manifest: &Manifest,
    env: &Option<Env>,
    idx: Option<usize>,
) {
    if let Some(i) = idx {
        table_divider!(table, "({i})");
    }

    if let Some(env) = env {
        table_columns!(table, "Project directory", env.project_dir.display());

        for package in &env.packages {
            table_columns!(table, "Package", &package.moniker);

            if let Some(obj) = package.props.as_object() {
                for (k, v) in obj {
                    table_line!(table, "{k}: {}", prettify_value(v));
                }
            }
        }
    } else {
        table_columns!(
            table,
            "Original project directory",
            manifest.original_project_dir().display()
        );
    }

    table_columns!(table, "Data directory", manifest.data_dir().display());
    table_columns!(table, "Manifest path", manifest.manifest_path().display());
    table_columns!(table, "Created at", manifest.created_at());
}

pub(crate) fn print_dir_info(table: &mut Table, dir_info: &DirInfo, env: &Option<Env>) {
    if let Some(env) = env {
        table_columns!(table, "Project directory", env.project_dir.display());

        for package in &env.packages {
            if let Ok(Some(env_info)) =
                Registry::global().make_env_info(dir_info.data_dir(), package, None)
            {
                table_columns!(table, "Package", &package.moniker);

                if let Some(obj) = package.props.as_object() {
                    for (k, v) in obj {
                        table_line!(table, "{k}: {}", prettify_value(v));
                    }
                }

                for p in env_info.path_dirs {
                    table_line!(table, "{}", p.display());
                }

                for (k, v) in env_info.vars {
                    table_line!(table, "{k} = {v}");
                }
            };
        }
    }

    table_columns!(table, "Data directory", dir_info.data_dir().display());
    table_columns!(table, "Manifest path", dir_info.manifest_path().display());
    table_columns!(table, "Data directory created at", dir_info.created_at());
    table_columns!(
        table,
        "Original project directory",
        dir_info.original_project_dir().display()
    );
    table_columns!(table, "Meta ID", dir_info.meta_id());
    table_columns!(table, "Link path", dir_info.link_path().display());
    table_columns!(table, "Link created at", dir_info.link_created_at());
    table_columns!(table, "Link ID", dir_info.link_id());
    table_columns!(table, "Project directory", dir_info.project_dir().display());
}

pub(crate) fn print_dir_info_and_env(table: &mut Table, dir_info: &DirInfo) -> Result<()> {
    table_title!(table, "Environment info");
    let env = existing(dir_info.read_env_config())?;
    print_dir_info(table, dir_info, &env);
    Ok(())
}

pub(crate) fn make_list_table() -> Table {
    TableSettings {
        divider_indent: 0,
        columns_indent: 2,
        column_colours: vec![Color::BrightYellow, Color::BrightWhite],
        column_separator: String::from("    "),
        ..Default::default()
    }
    .build()
}

pub(crate) fn make_prop_table() -> Table {
    TableSettings {
        title_indent: 0,
        divider_indent: 2,
        columns_indent: 2,
        line_indent: 4,
        divider_colour: Color::BrightWhite,
        column_colours: vec![Color::Green, Color::Yellow],
        column_separator: String::from(" : "),
        ..Default::default()
    }
    .build()
}

pub(crate) fn prettify_descriptor(plugin_host: &PluginHostRef, package: &Package) -> String {
    let descriptor_info = DescriptorInfo {
        plugin_host: Arc::clone(plugin_host),
        descriptor: Arc::clone(&package.descriptor),
    };
    descriptor_info.to_string()
}

pub(crate) fn prettify_package(package: &Package, verbose: bool) -> Result<String> {
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

pub(crate) fn prettify_value(value: &Value) -> String {
    if let Some(s) = value.as_str() {
        String::from(s)
    } else {
        value.to_string()
    }
}

#[allow(clippy::cast_precision_loss)]
pub(crate) fn humanize_size_base_2(bytes: u64) -> String {
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
