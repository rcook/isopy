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
use flate2::read::GzDecoder;
use joat_logger::{begin_operation, OpProgress};
use joatmon::open_file;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use tar::{Archive, Entry};

pub fn unpack_file(path: &Path, dir: &Path) -> Result<()> {
    fn unpack_entry(entry: &mut Entry<GzDecoder<File>>, path: PathBuf) -> Result<()> {
        let mut dir = path.clone();
        dir.pop();
        create_dir_all(&dir)?;
        entry.unpack(&path)?;
        Ok(())
    }

    // Open once to get number of entries (is this necessary?)
    let file = open_file(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let size = archive.entries()?.count();

    // Open a second time to unpack
    let file = open_file(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let op = begin_operation(Some(size as OpProgress))?;
    op.set_message(&format!("Unpacking {}", path.display()));

    for (idx, mut entry) in archive.entries()?.filter_map(|e| e.ok()).enumerate() {
        let path = dir.join(entry.path()?);
        unpack_entry(&mut entry, path)?;
        op.set_progress(idx as OpProgress);
    }

    drop(op);

    Ok(())
}
