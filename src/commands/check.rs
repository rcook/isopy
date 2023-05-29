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
use crate::print::{print_link, print_metadir};
use crate::status::Status;
use anyhow::Result;
use joat_repo::Trash;
use log::info;

pub fn do_check(app: &App, clean: bool) -> Result<Status> {
    let mut trash = Trash::compute(&app.repo)?;

    if trash.is_empty() {
        info!("No clean-up required");
        return Ok(Status::OK);
    }

    let invalid_link_count = trash.invalid_links.len();
    if invalid_link_count > 0 {
        if clean {
            println!("The following {invalid_link_count} links are invalid and will be removed:");
        } else {
            println!(
                "The following {invalid_link_count} links are invalid and can be removed with the --clean option:"
            );
        }

        for (idx, link) in trash.invalid_links.iter().enumerate() {
            println!("({})", idx + 1);
            print_link(link);
        }
    }

    let unreferenced_manifest_count = trash.unreferenced_manifests.len();
    if unreferenced_manifest_count > 0 {
        if clean {
            println!(
                "The following {unreferenced_manifest_count} metadirectories are unreferenced and will be removed:"
            );
        } else {
            println!(
                "The following {unreferenced_manifest_count} metadirectories are unreferenced and can be removed with the --clean option:"
            );
        }

        for (idx, manifest) in trash.unreferenced_manifests.iter().enumerate() {
            println!("({})", idx + 1);
            print_metadir(manifest, &None);
        }
    }

    if clean {
        trash.empty()?;
    }

    Ok(Status::OK)
}
