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
use crate::dir_info_ext::DirInfoExt;
use crate::plugin_host::PluginHostRef;
use crate::registry::Registry;
use crate::serialization::EnvRec;
use crate::util::{existing, pretty_descriptor};
use anyhow::{anyhow, Result};
use colored::Colorize;
use isopy_lib::{Package, PluginFactory};
use joat_repo::{DirInfo, Link, Manifest, Repo};
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::metadata;

pub fn print(s: &str) {
    println!("{}", s.bright_white());
}

pub fn print_error(s: &str) {
    println!("{}", s.bright_red());
}

pub fn print_title(label: &str) {
    println!("{}", label.cyan());
}

pub fn print_value<T>(label: &str, value: T)
where
    T: Display,
{
    println!("  {:30}: {}", label.green(), format!("{value}").yellow());
}

pub fn print_link(link: &Link) {
    print_value("Project directory", link.project_dir().display());
    print_value("Link path", link.link_path().display());
    print_value("Created at", link.created_at());
}

pub fn print_repo(repo: &Repo) {
    print_value("Lock file", repo.lock_path().display());
    print_value("Configuration file", repo.config_path().display());
    print_value("Links directory", repo.links_dir().display());
    print_value("Container directory", repo.container_dir().display());
    print_value("Shared directory", repo.shared_dir().display());
}

pub fn print_metadir(manifest: &Manifest, env_rec: &Option<EnvRec>) {
    if let Some(env_rec) = env_rec {
        print_value("Project directory", env_rec.project_dir.display());

        for package_rec in &env_rec.packages {
            print_value("Package", &package_rec.id);
            if let Ok(s) = serde_yaml::to_string(&package_rec.props) {
                for line in s.lines() {
                    println!("    {line}");
                }
            }
        }
    } else {
        print_value(
            "Original project directory",
            manifest.original_project_dir().display(),
        );
    }

    print_value("Data directory", manifest.data_dir().display());
    print_value("Manifest path", manifest.manifest_path().display());
    print_value("Created at", manifest.created_at());
}

pub fn print_dir_info(dir_info: &DirInfo, env_rec: &Option<EnvRec>) {
    if let Some(env_rec) = env_rec {
        print_value("Project directory", env_rec.project_dir.display());

        for package_rec in &env_rec.packages {
            if let Ok(Some(env_info)) =
                Registry::global().make_env_info(dir_info.data_dir(), package_rec, None)
            {
                print_value("Package", &package_rec.id);
                if let Ok(s) = serde_yaml::to_string(&package_rec.props) {
                    for line in s.lines() {
                        println!("    {line}");
                    }
                }
                for p in env_info.path_dirs {
                    println!("    {}", p.display());
                }
                for (k, v) in env_info.vars {
                    println!("    {k} = {v}");
                }
            };
        }
    }

    print_value("Data directory", dir_info.data_dir().display());
    print_value("Manifest path", dir_info.manifest_path().display());
    print_value("Data directory created at", dir_info.created_at());
    print_value(
        "Original project directory",
        dir_info.original_project_dir().display(),
    );
    print_value("Meta ID", dir_info.meta_id());
    print_value("Link path", dir_info.link_path().display());
    print_value("Link created at", dir_info.link_created_at());
    print_value("Link ID", dir_info.link_id());
    print_value("Project directory", dir_info.project_dir().display());
}

pub fn print_dir_info_and_env(dir_info: &DirInfo) -> Result<()> {
    print_title("Environment info");
    let env_rec = existing(dir_info.read_env_config())?;
    print_dir_info(dir_info, &env_rec);
    Ok(())
}

pub fn print_packages(
    plugin_host: &PluginHostRef,
    packages: &Vec<Package>,
    verbose: bool,
) -> Result<()> {
    if packages.is_empty() {
        print(&format!(
            "No packages found for {} ({})",
            plugin_host.name().cyan(),
            plugin_host.source_url().as_str().bright_magenta()
        ));
    } else {
        print(&format!(
            "{} ({})",
            plugin_host.name().cyan(),
            plugin_host.source_url().as_str().bright_magenta()
        ));

        for package in packages {
            let asset_pretty = if verbose && package.asset_path.is_file() {
                let m = metadata(&package.asset_path)?;
                let asset_path_pretty = format!("{}", package.asset_path.display()).bright_white();
                let size_pretty = format!("{}", humanize_size_base_2(m.len()).cyan());
                format!("{asset_path_pretty} ({size_pretty})")
                    .bright_white()
                    .bold()
            } else {
                package
                    .asset_path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| anyhow!("cannot convert file name"))?
                    .white()
            };

            print(&format!(
                "  {:<30} {}",
                pretty_descriptor(plugin_host, package).bright_yellow(),
                asset_pretty
            ));
        }
    }

    Ok(())
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
