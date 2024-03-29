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
use crate::response::{ContentLength, Response};
use anyhow::Result;
use joat_logger::begin_operation;
use joatmon::{safe_back_up, safe_create_file};
use log::info;
use std::fs::remove_file;
use std::io::Write;
use std::path::Path;

pub async fn download_stream(
    label: &str,
    response: &mut Box<dyn Response>,
    output_path: &Path,
) -> Result<()> {
    if output_path.exists() {
        let safe_back_up_path = safe_back_up(output_path)?;
        info!(
            "Data file {} backed up to {}",
            output_path.display(),
            safe_back_up_path.display()
        );
        remove_file(output_path)?;
    }

    let op = begin_operation(response.content_length())?;
    let mut stream = response.bytes_stream()?;
    let mut file = safe_create_file(output_path, false)?;
    let mut downloaded = 0;
    op.set_message(&format!("Fetching {label}"));
    while let Some(item) = stream.next().await {
        let chunk = item?;
        downloaded += chunk.len() as ContentLength;
        file.write_all(&chunk)?;
        op.set_progress(downloaded);
    }
    op.set_message(&format!("Finished fetching {label}"));
    drop(op);
    Ok(())
}
