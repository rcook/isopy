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
use super::super::Status;
use anyhow::Result;
use colored::Colorize;
use joat_repo::Repo;

pub fn do_list(repo: &Repo) -> Result<Status> {
    let mut manifests = repo.list_manifests()?;
    manifests.sort_by_cached_key(|m| m.meta_id().clone());
    let manifests = manifests;
    if !manifests.is_empty() {
        println!(
            "{}",
            format!("Metadirectories ({})", manifests.len()).green()
        );
        for manifest in manifests {
            println!(
                "  {} ({})",
                manifest.meta_id().to_string().yellow(),
                manifest.data_dir().display().to_string().blue()
            );
        }
    }

    let mut links = repo.list_links()?;
    links.sort_by_cached_key(|l| l.project_dir().to_path_buf());
    let links = links;
    if !links.is_empty() {
        println!("{}", format!("Links ({})", links.len()).green());
        for link in links {
            println!(
                "  {} ({})\n    -> {} ({})",
                link.link_id().to_string().yellow(),
                link.link_path().display().to_string().blue(),
                link.meta_id().to_string().yellow(),
                link.project_dir().display().to_string().bright_magenta(),
            );
        }
    }

    Ok(Status::Success)
}
