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
use crate::serialization::Manifest;
use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Cache {
    pub(crate) path: PathBuf,
    pub(crate) manifest: Manifest,
}

impl Cache {
    pub(crate) fn load<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let manifest = if path.is_file() {
            let f = File::open(&path)?;
            serde_yaml::from_reader(f)?
        } else {
            Manifest::default()
        };
        Ok(Self { path, manifest })
    }

    pub(crate) fn save(&self) -> Result<()> {
        let f = File::create(&self.path)?;
        serde_yaml::to_writer(f, &self.manifest)?;
        Ok(())
    }
}
