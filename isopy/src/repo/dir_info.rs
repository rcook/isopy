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
use crate::repo::link::Link;
use crate::repo::link_id::LinkId;
use crate::repo::manifest::Manifest;
use crate::repo::meta_id::MetaId;
use chrono::{DateTime, Utc};
use std::path::Path;

#[derive(Debug)]
pub struct DirInfo {
    pub(crate) manifest: Manifest,
    pub(crate) link: Link,
}

impl DirInfo {
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        self.manifest.data_dir()
    }

    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        self.manifest.manifest_path()
    }

    #[must_use]
    pub const fn created_at(&self) -> &DateTime<Utc> {
        self.manifest.created_at()
    }

    #[must_use]
    pub fn original_project_dir(&self) -> &Path {
        self.manifest.original_project_dir()
    }

    #[must_use]
    pub const fn meta_id(&self) -> &MetaId {
        self.manifest.meta_id()
    }

    #[must_use]
    pub fn link_path(&self) -> &Path {
        self.link.link_path()
    }

    #[must_use]
    pub const fn link_created_at(&self) -> &DateTime<Utc> {
        self.link.created_at()
    }

    #[must_use]
    pub const fn link_id(&self) -> &LinkId {
        self.link.link_id()
    }

    #[must_use]
    pub fn project_dir(&self) -> &Path {
        self.link.project_dir()
    }
}
