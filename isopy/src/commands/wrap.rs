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
use crate::constants::ENV_FILE_NAME;
use crate::serialization::EnvRec;
use crate::status::Status;
use anyhow::{bail, Result};
#[allow(unused)]
use joatmon::{read_yaml_file, safe_write_file};
use serde::Serialize;
use std::env::join_paths;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
#[allow(unused)]
use tinytemplate::TinyTemplate;

#[allow(unused)]
#[cfg(any(target_os = "linux", target_os = "macos"))]
const PYTHON_EXECUTABLE_NAME: &str = "python3";

#[allow(unused)]
#[cfg(target_os = "windows")]
const PYTHON_EXECUTABLE_NAME: &str = "python";

#[allow(unused)]
#[cfg(any(target_os = "linux", target_os = "macos"))]
const WRAPPER_TEMPLATE: &str = r#"#!/bin/bash
set -euo pipefail
{path_env} \
PYTHONPATH={base_dir} \
exec {python_executable_name} {script_path} "$@""#;

#[allow(unused)]
#[cfg(target_os = "windows")]
const WRAPPER_TEMPLATE: &str = r#"@echo off
setlocal
{path_env}
set PYTHONPATH={base_dir}
{python_executable_name} "{script_path}" %*"#;

#[derive(Serialize)]
struct Context {
    path_env: String,
    base_dir: PathBuf,
    python_executable_name: PathBuf,
    script_path: PathBuf,
}

#[allow(unused)]
pub fn do_wrap(
    app: &App,
    wrapper_path: &Path,
    script_path: &Path,
    base_dir: &Path,
) -> Result<Status> {
    let Some(dir_info) = app.find_dir_info(&app.cwd, None)? else {
        bail!("Could not find environment for directory {}", app.cwd.display())
    };

    let data_dir = dir_info.data_dir();
    let env_rec = read_yaml_file::<EnvRec>(&data_dir.join(ENV_FILE_NAME))?;

    todo!()
    /*
    let python_dir = data_dir.join(rec.dir);

    let mut template = TinyTemplate::new();
    template.add_template("WRAPPER", WRAPPER_TEMPLATE)?;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let path_env = make_path_env(&[&python_dir.join("bin")])?;

    #[cfg(any(target_os = "windows"))]
    let path_env = make_path_env(&[&python_dir, &python_dir.join("Scripts")])?;

    let Some(path_env_str) = path_env.to_str() else {
        bail!("failed to generate PATH environment variable");
    };

    safe_write_file(
        wrapper_path,
        template.render(
            "WRAPPER",
            &Context {
                path_env: String::from(path_env_str),
                base_dir: base_dir.to_path_buf(),
                python_executable_name: PathBuf::from(PYTHON_EXECUTABLE_NAME),
                script_path: script_path.to_path_buf(),
            },
        )?,
        false,
    )?;

    set_file_attributes(wrapper_path)?;

    Ok(Status::OK)
    */
}

#[allow(unused)]
fn make_path_env(paths: &[&Path]) -> Result<OsString> {
    let mut new_paths = paths.to_vec();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    new_paths.push(Path::new("$PATH"));

    #[cfg(any(target_os = "windows"))]
    new_paths.push(Path::new("%PATH%"));

    let mut s = OsString::new();
    s.push("PATH=");
    s.push(join_paths(new_paths)?);
    Ok(s)
}

#[allow(unused)]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn set_file_attributes(wrapper_path: &Path) -> Result<()> {
    use std::fs::{metadata, set_permissions};
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = metadata(wrapper_path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o100);
    set_permissions(wrapper_path, permissions)?;
    Ok(())
}

#[allow(unused)]
#[cfg(target_os = "windows")]
fn set_file_attributes(_wrapper_path: &Path) -> Result<()> {
    Ok(())
}
