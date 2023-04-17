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
use crate::app::App;
use crate::object_model::{EnvironmentName, Tag, Version};
use crate::serialization::NamedEnvironmentRecord;
use crate::util::{get_asset, unpack_file};
use anyhow::Result;
use joatmon::safe_write_file;
use std::path::PathBuf;

pub async fn do_create(
    app: &App,
    environment_name: &EnvironmentName,
    version: &Version,
    tag: &Option<Tag>,
) -> Result<()> {
    let assets = app.read_assets()?;
    let asset = get_asset(&assets, version, tag)?;

    let archive_path = app.assets_dir.join(&asset.name);
    let dir = app.named_environment_dir(environment_name);
    unpack_file(archive_path, &dir)?;

    safe_write_file(
        dir.join("env.yaml"),
        serde_yaml::to_string(&NamedEnvironmentRecord {
            name: environment_name.clone(),
            python_dir_rel: PathBuf::from("python"),
            python_version: asset.meta.version.clone(),
            tag: asset.tag.clone(),
        })?,
        false,
    )?;

    Ok(())
}
