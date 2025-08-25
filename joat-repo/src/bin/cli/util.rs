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
use anyhow::Result;
use colored::Colorize;
use joat_repo::{DirInfo, Link, Manifest};
use std::fmt::Display;
use std::io::{stdin, stdout, Write};

pub fn prompt(message: &str) -> Result<String> {
    print!("{message}: ");
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    Ok(line.trim().to_lowercase())
}

pub fn print<T>(label: &str, value: T)
where
    T: Display,
{
    println!("{:30}: {}", label.green(), format!("{value}").yellow());
}

pub fn print_data_dir(dir_info: &DirInfo) {
    print("Data directory", dir_info.data_dir().display());
    print("Manifest path", dir_info.manifest_path().display());
    print("Data directory created at", dir_info.created_at());
    print(
        "Original project directory",
        dir_info.original_project_dir().display(),
    );
    print("Meta ID", dir_info.meta_id());
    print("Link path", dir_info.link_path().display());
    print("Link created at", dir_info.link_created_at());
    print("Link ID", dir_info.link_id());
    print("Project directory", dir_info.project_dir().display());
}

pub fn print_manifest(manifest: &Manifest) {
    print("Data directory", manifest.data_dir().display());
    print("Manifest path", manifest.manifest_path().display());
    print("Created at", manifest.created_at());
    print(
        "Original project directory",
        manifest.original_project_dir().display(),
    );
    print("Meta ID", manifest.meta_id());
}

pub fn print_link(link: &Link) {
    print("Link path", link.link_path().display());
    print("Created at", link.created_at());
    print("Link ID", link.link_id());
    print("Project directory", link.project_dir().display());
    print("Meta ID", link.meta_id());
}
