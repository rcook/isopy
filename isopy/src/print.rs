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
use crate::app::App;
use crate::dir_info_ext::DirInfoExt;
use crate::fs::existing;
use crate::serialization::Env;
use crate::table::{table_columns, table_divider, table_line, table_title, Table, TableSettings};
use anyhow::Result;
use colored::Color;
use joat_repo::{DirInfo, Link, Manifest, Repo};

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
            table_line!(table, "dir: {}", package.dir.display());
            table_line!(table, "url: {}", package.url);
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

pub(crate) fn print_dir_info(app: &App, table: &mut Table, dir_info: &DirInfo, env: &Option<Env>) {
    if let Some(env) = env {
        table_columns!(table, "Project directory", env.project_dir.display());

        for package in &env.packages {
            if let Ok(env_info) = app.make_env_info(dir_info.data_dir(), package, None) {
                table_columns!(table, "Package", &package.moniker);

                table_line!(table, "dir: {}", package.dir.display());
                table_line!(table, "url: {}", package.url);

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

pub(crate) fn print_dir_info_and_env(
    app: &App,
    table: &mut Table,
    dir_info: &DirInfo,
) -> Result<()> {
    table_title!(table, "Environment information");
    let env = existing(dir_info.read_env_config())?;
    print_dir_info(app, table, dir_info, &env);
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
