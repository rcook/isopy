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
use crate::constants::ENV_FILE_NAME;
use crate::serialization::EnvRec;
use anyhow::Result;
use colored::Colorize;
use joat_repo::{DirInfo, Link, Manifest, Repo};
use joatmon::read_yaml_file;
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
    println!("  {:30}: {}", label.green(), format!("{}", value).yellow());
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

pub fn print_metadir(manifest: &Manifest, rec_opt: &Option<EnvRec>) {
    if let Some(rec) = rec_opt {
        print_value("Project path", rec.config_path.display());
        print_value("Python version", rec.version.as_str());
        print_value("Python build tag", rec.tag.as_str());
        print_value("Python directory", rec.python_dir_rel.display());
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

pub fn print_dir_info(dir_info: &DirInfo, rec_opt: &Option<EnvRec>) {
    if let Some(rec) = rec_opt {
        print_value("Project path", rec.config_path.display());
        print_value("Python version", rec.version.as_str());
        print_value("Python build tag", rec.tag.as_str());
        print_value("Python directory", rec.python_dir_rel.display());
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

    let env_yaml_path = dir_info.data_dir().join(ENV_FILE_NAME);
    let rec_opt = if env_yaml_path.is_file() {
        Some(read_yaml_file::<EnvRec>(&env_yaml_path)?)
    } else {
        None
    };

    print_dir_info(dir_info, &rec_opt);

    Ok(())
}
