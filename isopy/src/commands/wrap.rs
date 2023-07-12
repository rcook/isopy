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
use crate::status::Status;
use anyhow::anyhow;
use anyhow::Result;
use joatmon::safe_write_file;
use log::{error, info};
use serde::Serialize;
use std::env::join_paths;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use tinytemplate::TinyTemplate;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const WRAPPER_TEMPLATE: &str = r#"#!/bin/bash
set -euo pipefail
{path_env} \
{vars}exec {command} "$@"
"#;

#[cfg(target_os = "windows")]
const WRAPPER_TEMPLATE: &str = r#"@echo off
setlocal
{path_env}
{vars}
"{command}" %*
"#;

#[derive(Serialize)]
struct TemplateContext {
    path_env: String,
    vars: String,
    command: String,
}

pub enum WrapTarget {
    Command(String),
    Script(PathBuf),
}

pub fn wrap(
    app: &App,
    wrapper_file_name: &OsStr,
    target: &WrapTarget,
    base_dir: &Path,
    force: bool,
) -> Result<Status> {
    let Some(dir_info) = app.find_dir_info(&app.cwd, None)? else {
        error!("could not find environment for directory {}", app.cwd.display());
        return Ok(Status::Fail);
    };

    let Some(env_info) = App::make_env_info(dir_info.data_dir(), Some(base_dir))? else {
        error!("could not get environment info");
        return Ok(Status::Fail);
    };

    let mut template = TinyTemplate::new();
    template.add_template("WRAPPER", WRAPPER_TEMPLATE)?;

    let path_env = String::from(
        make_path_env(&env_info.path_dirs)?
            .to_str()
            .ok_or_else(|| anyhow!("failed to generate PATH environment variable"))?,
    );

    let vars = env_info
        .vars
        .iter()
        .map(|(k, v)| format!("{k}={v} \\\n"))
        .collect::<String>();

    let wrapper_path = app.cache_dir.join("bin").join(wrapper_file_name);

    let command = match target {
        WrapTarget::Command(s) => s.clone(),
        WrapTarget::Script(p) => {
            String::from(p.to_str().ok_or_else(|| anyhow!("cannot convert path"))?)
        }
    };

    let s = template.render(
        "WRAPPER",
        &TemplateContext {
            path_env,
            vars,
            command,
        },
    )?;

    safe_write_file(&wrapper_path, s, force)?;
    set_file_attributes(&wrapper_path)?;
    info!("wrapper created at {}", wrapper_path.display());
    Ok(Status::OK)
}

fn make_path_env(paths: &[PathBuf]) -> Result<OsString> {
    let mut new_paths = paths.to_owned();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    new_paths.push(PathBuf::from("$PATH"));

    #[cfg(any(target_os = "windows"))]
    new_paths.push(PathBuf::from("%PATH%"));

    let mut s = OsString::new();
    s.push("PATH=");
    s.push(join_paths(new_paths)?);
    Ok(s)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn set_file_attributes(wrapper_path: &Path) -> Result<()> {
    use std::fs::{metadata, set_permissions};
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = metadata(wrapper_path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o100);
    set_permissions(wrapper_path, permissions)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_file_attributes(_wrapper_path: &Path) -> Result<()> {
    Ok(())
}
