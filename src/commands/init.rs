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
use crate::object_model::ProductDescriptor;
use crate::status::Status;
use crate::unpack::unpack_file;
use crate::{app::App, object_model::OpenJdkProductDescriptor};
use anyhow::{bail, Result};
use std::path::PathBuf;

pub async fn do_init(app: &App, product_descriptor: &ProductDescriptor) -> Result<Status> {
    if app.repo.get(&app.cwd)?.is_some() {
        bail!("Directory {} already has environment", app.cwd.display())
    }

    match product_descriptor {
        ProductDescriptor::Python(d) => app.init_project(d).await?,
        ProductDescriptor::OpenJdk(d) => do_init_openjdk(app, d).await?,
    }

    Ok(Status::OK)
}

async fn do_init_openjdk(app: &App, product_descriptor: &OpenJdkProductDescriptor) -> Result<()> {
    let asset_path = app.download_openjdk(product_descriptor).await?;

    //unpack_file(&asset_path, dir_info.data_dir())?;
    unpack_file(&asset_path, &PathBuf::from("XYZ"))?;

    Ok(())
}
