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
use joat_repo::{DirInfo, Link, Repo};
use std::collections::HashMap;
use std::path::Path;

fn find_link(repo: &Repo, dir: &Path) -> Result<Option<Link>> {
    let mut map = repo
        .list_links()?
        .into_iter()
        .map(|x| (x.project_dir().to_path_buf(), x))
        .collect::<HashMap<_, _>>();

    let mut d = dir;
    loop {
        if let Some(link) = map.remove(d) {
            return Ok(Some(link));
        }

        if let Some(p) = d.parent() {
            d = p;
        } else {
            return Ok(None);
        }
    }
}

pub fn find_dir_info(repo: &Repo, cwd: &Path) -> Result<Option<DirInfo>> {
    let Some(link) = find_link(repo, cwd)? else {
        return Ok(None)
    };

    let Some(dir_info) = repo.get(link.project_dir())? else {
        return Ok(None)
    };

    Ok(Some(dir_info))
}