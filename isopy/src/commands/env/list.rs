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
use crate::status::Status;
use crate::table::{table_divider, table_title};
use anyhow::Result;

pub fn list(app: &App) -> Result<Status> {
    let mut table = make_prop_table();

    let manifests = app.repo.list_manifests()?;
    if !manifests.is_empty() {
        table_title!(table, "Metadirectories");
        for (idx, manifest) in manifests.iter().enumerate() {
            table_divider!(table, "({}) {}", idx + 1, manifest.meta_id());
            let env_rec = existing(manifest.read_env_config())?;
            print_metadir(&mut table, manifest, &env_rec, None);
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
                link.link_id(),
                link.meta_id()
            );

            print_link(&mut table, link, None);
        }
    }

    table.print();

    Ok(Status::Success)
}
