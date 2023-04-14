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
use crate::object_model::{Tag, Version};
use crate::serialization::ProjectRecord;
use crate::util::{get_asset, make_project_config_path};
use anyhow::Result;
use swiss_army_knife::safe_write_file;

pub fn do_new(app: &App, version: &Version, tag: &Option<Tag>) -> Result<()> {
    let project_config_path = make_project_config_path(&app.cwd);
    let project_record = ProjectRecord {
        python_version: version.clone(),
        tag: tag.clone(),
    };

    // Make sure there's exactly one asset matching this Python version and tag
    let assets = app.read_assets()?;
    _ = get_asset(&assets, &project_record.python_version, &project_record.tag)?;

    let s = serde_yaml::to_string(&project_record)?;
    safe_write_file(&project_config_path, s, false)?;
    println!(
        "Wrote project configuration file to {}",
        project_config_path.display()
    );

    Ok(())
}
