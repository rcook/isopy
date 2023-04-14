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
use crate::object_model::{Environment, EnvironmentName};
use crate::serialization::UseRecord;
use anyhow::{bail, Result};
use swiss_army_knife::safe_write_file;

pub fn do_use(app: &App, environment_name: &EnvironmentName) -> Result<()> {
    let use_yaml_path = app.use_dir(&app.cwd)?.join("use.yaml");
    if use_yaml_path.is_file() {
        bail!(
            "Use is already configured for directory {}",
            app.cwd.display()
        )
    }

    let environment = Environment::infer(app, Some(environment_name))?;

    safe_write_file(
        use_yaml_path,
        serde_yaml::to_string(&UseRecord {
            dir: app.cwd.clone(),
            environment_name: environment.name,
        })?,
        false,
    )?;
    Ok(())
}
