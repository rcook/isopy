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
use super::super::util::print_data_dir;
use super::super::{prompt, Status};
use anyhow::Result;
use joat_repo::{MetaId, Repo};
use log::error;
use std::path::Path;

fn prompt_for_meta_id(repo: &Repo) -> Result<Option<MetaId>> {
    let manifests = repo.list_manifests()?;
    let manifest_count = manifests.len();

    if manifest_count == 0 {
        error!("No metadirectories exist yet!");
        return Ok(None);
    }

    for (idx, manifest) in manifests.iter().enumerate() {
        println!(
            "({}): {}: {}",
            idx + 1,
            manifest.meta_id(),
            manifest.original_project_dir().display()
        );
    }

    let line = if manifest_count > 1 {
        prompt(&format!("Enter 1-{manifest_count} or Q to quit"))?
    } else {
        prompt("Enter 1 or Q to quit")?
    };
    if line == "q" {
        return Ok(None);
    }

    let Ok(index) = line.parse::<usize>() else {
        return Ok(None);
    };

    if index < 1 || index > manifest_count {
        return Ok(None);
    }

    Ok(Some(manifests[index - 1].meta_id().clone()))
}

pub fn do_link(repo: &Repo, meta_id: &Option<MetaId>, cwd: &Path) -> Result<Status> {
    if let Some(link) = repo.read_link(cwd)? {
        error!(
            "Link {} already exists for directory {}",
            link.link_id(),
            cwd.display()
        );
        return Ok(Status::Failure);
    }

    let meta_id = match meta_id {
        Some(value) => value.clone(),
        None => match prompt_for_meta_id(repo)? {
            Some(value) => value,
            None => return Ok(Status::Failure),
        },
    };

    Ok(if let Some(dir_info) = repo.link(&meta_id, cwd)? {
        print_data_dir(&dir_info);
        Status::Success
    } else {
        error!("Could not create link for directory {}", cwd.display());
        Status::Failure
    })
}
