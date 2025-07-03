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
use crate::print::{make_prop_table, print_link, print_metadir};
use crate::status::{success, StatusResult};
use crate::table::{table_divider, table_title};
use anyhow::Result;
use colored::Colorize;
use log::info;

pub(crate) fn do_list(app: &App, verbose: bool) -> StatusResult {
    if verbose {
        list_verbose(app)?;
    } else {
        list_brief(app)?;
    }

    success!();
}

fn list_verbose(app: &App) -> Result<()> {
    let mut table = make_prop_table();

    let manifests = app.repo.list_manifests()?;
    if !manifests.is_empty() {
        table_title!(table, "Package directories");
        for (idx, manifest) in manifests.iter().enumerate() {
            table_divider!(
                table,
                "({}) {}",
                idx + 1,
                manifest.meta_id().to_string().bright_cyan()
            );
            let env = existing(manifest.read_env_config())?;
            print_metadir(&mut table, manifest, env.as_ref(), None);
        }
    }

    let links = app.repo.list_links()?;
    if !links.is_empty() {
        table_title!(table, "Links");

        for (idx, link) in links.iter().enumerate() {
            table_divider!(
                table,
                "({}) {} -> {}",
                idx + 1,
                link.link_id().to_string().bright_magenta(),
                link.meta_id().to_string().bright_cyan()
            );

            print_link(&mut table, link, None);
        }
    }

    table.print();
    Ok(())
}

fn list_brief(app: &App) -> Result<()> {
    let manifests = app.repo.list_manifests()?;
    if manifests.is_empty() {
        info!("You have no metadirectories");
    } else {
        info!("You have {} metadirectories:", manifests.len());
        for (idx, manifest) in manifests.iter().enumerate() {
            info!("({}) {}", idx + 1, manifest.data_dir().display());
        }
    }

    let links = app.repo.list_links()?;
    if links.is_empty() {
        info!("You have no project directories linked to metadirectories");
    } else {
        info!(
            "You have {} project directories linked to metadirectories:",
            links.len()
        );
        for (idx, link) in links.iter().enumerate() {
            info!(
                "({}) {} -> {}",
                idx + 1,
                link.project_dir().display(),
                link.meta_id()
            );
        }
    }

    info!("Pass --verbose to get more information");

    Ok(())
}
