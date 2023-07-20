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
use crate::registry::Registry;
use crate::serialization::EnvRec;
use crate::table::{table_row, Table, TableSettings};
use crate::util::existing;
use anyhow::Result;
use colored::Color;
use joat_repo::{DirInfo, Link, Manifest, Repo};

pub fn print_link(table: &mut Table, link: &Link, idx: Option<usize>) {
    if let Some(i) = idx {
        table.add_divider(&format!("({i})"));
    }

    table_row!(table, "Project directory", link.project_dir().display());
    table_row!(table, "Link path", link.link_path().display());
    table_row!(table, "Created at", link.created_at());
}

pub fn print_repo(table: &mut Table, repo: &Repo) {
    table_row!(table, "Lock file", repo.lock_path().display());
    table_row!(table, "Configuration file", repo.config_path().display());
    table_row!(table, "Links directory", repo.links_dir().display());
    table_row!(table, "Container directory", repo.container_dir().display());
    table_row!(table, "Shared directory", repo.shared_dir().display());
}

pub fn print_metadir(
    table: &mut Table,
    manifest: &Manifest,
    env_rec: &Option<EnvRec>,
    idx: Option<usize>,
) {
    if let Some(i) = idx {
        table.add_divider(&format!("({i})"));
    }

    if let Some(env_rec) = env_rec {
        table_row!(table, "Project directory", env_rec.project_dir.display());

        for package_rec in &env_rec.packages {
            table_row!(table, "Package", &package_rec.id);

            if let Some(obj) = package_rec.props.as_object() {
                for (k, v) in obj {
                    table.add_line(&format!("{k}: {v}"));
                }
            }
        }
    } else {
        table_row!(
            table,
            "Original project directory",
            manifest.original_project_dir().display()
        );
    }

    table_row!(table, "Data directory", manifest.data_dir().display());
    table_row!(table, "Manifest path", manifest.manifest_path().display());
    table_row!(table, "Created at", manifest.created_at());
}

pub fn print_dir_info(table: &mut Table, dir_info: &DirInfo, env_rec: &Option<EnvRec>) {
    if let Some(env_rec) = env_rec {
        table_row!(table, "Project directory", env_rec.project_dir.display());

        for package_rec in &env_rec.packages {
            if let Ok(Some(env_info)) =
                Registry::global().make_env_info(dir_info.data_dir(), package_rec, None)
            {
                table_row!(table, "Package", &package_rec.id);

                if let Some(obj) = package_rec.props.as_object() {
                    for (k, v) in obj {
                        table.add_line(&format!("{k}: {v}"));
                    }
                }

                for p in env_info.path_dirs {
                    table.add_line(&p.display().to_string());
                }

                for (k, v) in env_info.vars {
                    table.add_line(&format!("{k} = {v}"));
                }
            };
        }
    }

    table_row!(table, "Data directory", dir_info.data_dir().display());
    table_row!(table, "Manifest path", dir_info.manifest_path().display());
    table_row!(table, "Data directory created at", dir_info.created_at());
    table_row!(
        table,
        "Original project directory",
        dir_info.original_project_dir().display()
    );
    table_row!(table, "Meta ID", dir_info.meta_id());
    table_row!(table, "Link path", dir_info.link_path().display());
    table_row!(table, "Link created at", dir_info.link_created_at());
    table_row!(table, "Link ID", dir_info.link_id());
    table_row!(table, "Project directory", dir_info.project_dir().display());
}

pub fn print_dir_info_and_env(table: &mut Table, dir_info: &DirInfo) -> Result<()> {
    table.add_title("Environment info");
    let env_rec = existing(dir_info.read_env_config())?;
    print_dir_info(table, dir_info, &env_rec);
    Ok(())
}

pub fn make_list_table() -> Table {
    TableSettings {
        divider_indent: 0,
        columns_indent: 2,
        column_colours: vec![Color::BrightYellow, Color::BrightWhite],
        column_separator: String::from("    "),
        ..Default::default()
    }
    .build()
}

pub fn make_prop_table() -> Table {
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
