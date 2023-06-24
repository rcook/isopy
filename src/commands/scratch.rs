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
use crate::adoptium::AdoptiumIndexManager;
use crate::app::App;
use crate::constants::{ADOPTIUM_INDEX_FILE_NAME, ADOPTIUM_SERVER_URL};
use crate::object_model::OpenJdkVersion;
use crate::status::Status;
use anyhow::{bail, Result};

pub async fn do_scratch(app: &App, openjdk_version: &OpenJdkVersion) -> Result<Status> {
    let manager = AdoptiumIndexManager::new(
        &ADOPTIUM_SERVER_URL,
        &app.repo.shared_dir().join(ADOPTIUM_INDEX_FILE_NAME),
    );

    let versions = manager.read_versions().await?;
    let Some(version) = versions
        .iter()
        .find(|x| x.openjdk_version == *openjdk_version) else {
        bail!("no version matching {}", openjdk_version);
    };

    let output_path = app.repo.shared_dir().join(&version.file_name);
    if output_path.exists() {
        println!("{} already downloaded", version.file_name.display());
    } else {
        manager.download_asset(&version.url, &output_path).await?;
    }

    Ok(Status::OK)
}
