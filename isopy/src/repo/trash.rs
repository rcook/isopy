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
use crate::repo::Repo;
use crate::repo::link::Link;
use crate::repo::manifest::Manifest;
use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{remove_dir_all, remove_file};

#[derive(Debug)]
pub struct Trash {
    pub unreferenced_manifests: Vec<Manifest>,
    pub invalid_links: Vec<Link>,
}

struct ManifestStatus {
    manifest: Manifest,
    is_referenced: bool,
}

struct LinkStatus {
    link: Link,
    is_valid: bool,
}

impl Trash {
    pub fn compute(repo: &Repo) -> Result<Self> {
        let mut manifest_map = repo
            .list_manifests()?
            .into_iter()
            .map(|m| {
                (
                    m.meta_id().clone(),
                    ManifestStatus {
                        manifest: m,
                        is_referenced: false,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let mut link_map = repo
            .list_links()?
            .into_iter()
            .map(|l| {
                (
                    l.link_id().clone(),
                    LinkStatus {
                        link: l,
                        is_valid: true,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        for l in link_map.values_mut() {
            if l.link.project_dir().is_dir() {
                match manifest_map.get_mut(l.link.meta_id()) {
                    Some(m) => m.is_referenced = true,
                    None => l.is_valid = false,
                }
            } else {
                l.is_valid = false;
            }
        }

        let invalid_links = link_map
            .into_values()
            .filter(|x| !x.is_valid)
            .map(|x| x.link)
            .collect::<Vec<_>>();
        let unreferenced_manifests = manifest_map
            .into_values()
            .filter(|x| !x.is_referenced)
            .map(|x| x.manifest)
            .collect::<Vec<_>>();

        Ok(Self {
            unreferenced_manifests,
            invalid_links,
        })
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.invalid_links.len() + self.unreferenced_manifests.len() == 0
    }

    pub fn empty(&mut self) -> Result<()> {
        for l in self.invalid_links.drain(..) {
            remove_file(l.link_path())?;
        }

        for m in self.unreferenced_manifests.drain(..) {
            remove_dir_all(m.data_dir())?;
        }

        Ok(())
    }
}
