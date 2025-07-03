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
use crate::print::{make_prop_table, print_link, print_metadir};
use crate::status::{success, StatusResult};
use crate::table::table_divider;
use joat_repo::Trash;

pub(crate) fn do_check(app: &App, clean: bool) -> StatusResult {
    let mut trash = Trash::compute(&app.repo)?;

    if trash.is_empty() {
        success!("no clean-up required");
    }

    let mut table = make_prop_table();

    let invalid_link_count = trash.invalid_links.len();
    if invalid_link_count > 0 {
        if clean {
            table_divider!(
                table,
                "The following {invalid_link_count} links are invalid and will be removed:"
            );
        } else {
            table_divider!(table,"The following {invalid_link_count} links are invalid and can be removed with the --clean option:");
        }

        for (idx, link) in trash.invalid_links.iter().enumerate() {
            print_link(&mut table, link, Some(idx + 1));

            table.print();
        }
    }

    let unreferenced_manifest_count = trash.unreferenced_manifests.len();
    if unreferenced_manifest_count > 0 {
        if clean {
            table_divider!(table, "The following {unreferenced_manifest_count} metadirectories are unreferenced and will be removed:");
        } else {
            table_divider!(table, "The following {unreferenced_manifest_count} metadirectories are unreferenced and can be removed with the --clean option:");
        }

        for (idx, manifest) in trash.unreferenced_manifests.iter().enumerate() {
            print_metadir(&mut table, manifest, None, Some(idx + 1));
        }
    }

    table.print();

    if clean {
        trash.empty()?;
    }

    success!();
}
