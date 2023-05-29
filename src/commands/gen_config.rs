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
use crate::args::PythonVersion;
use crate::constants::PYTHON_VERSION_FILE_NAME;
use crate::serialization::PythonVersionRec;
use crate::status::Status;
use anyhow::Result;
use joatmon::safe_write_file;

pub fn do_gen_config(app: &App, python_version: &PythonVersion, force: bool) -> Result<Status> {
    let config_path = app.cwd.join(PYTHON_VERSION_FILE_NAME);

    let rec = PythonVersionRec {
        version: python_version.version.clone(),
        tag: python_version.tag.clone(),
    };

    let yaml_str = serde_yaml::to_string(&rec)?;

    safe_write_file(&config_path, yaml_str, force)?;

    Ok(Status::OK)
}
