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
use crate::constants::ENV_CONFIG_FILE_NAME;
use crate::plugin_host::PluginHostRef;
use crate::registry::Registry;
use crate::serialization::EnvRec;
use crate::util::pretty_descriptor;
use anyhow::Result;
use colored::Colorize;
use isopy_lib::{Package, PluginFactory};
use joat_repo::{DirInfo, Link, Manifest, Repo};
use joatmon::read_yaml_file;
use std::ffi::OsStr;
use std::fmt::Display;

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
                Registry::global().make_env_info(dir_info.data_dir(), package_rec)
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
                for (k, v) in env_info.envs {
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

    let env_yaml_path = dir_info.data_dir().join(&*ENV_CONFIG_FILE_NAME);
    let env_rec = if env_yaml_path.is_file() {
        Some(read_yaml_file::<EnvRec>(&env_yaml_path)?)
    } else {
        None
    };

    print_dir_info(dir_info, &env_rec);

    Ok(())
}

pub fn print_packages(plugin_host: &PluginHostRef, packages: &Vec<Package>, verbose: bool) {
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
            if verbose {
                print(&format!(
                    "  {:<30} {}",
                    pretty_descriptor(plugin_host, package).bright_yellow(),
                    package.asset_path.display()
                ));
            } else {
                print(&format!(
                    "  {:<30} {}",
                    pretty_descriptor(plugin_host, package).bright_yellow(),
                    package
                        .asset_path
                        .file_name()
                        .and_then(OsStr::to_str)
                        .expect("must be valid")
                ));
            }
        }
    }
}
