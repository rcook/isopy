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
use crate::dir_info_ext::DirInfoExt;
use crate::fs::{ensure_file_executable_mode, is_executable_file};
use crate::status::{return_success, return_user_error, Status};
use crate::wrapper_file_name::WrapperFileName;
use anyhow::{anyhow, bail, Result};
use isopy_lib::{Platform, Shell};
use joat_repo::DirInfo;
use joatmon::safe_write_file;
use log::info;
use serde::Serialize;
use std::env::join_paths;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use tinytemplate::{format_unescaped, TinyTemplate};

const BASH_WRAPPER_TEMPLATE: &str = r#"#!/bin/bash
set -euo pipefail
{path_env} \
{vars}exec {command} "$@"
"#;

const CMD_WRAPPER_TEMPLATE: &str = r#"@echo off
setlocal
{path_env}
{vars}
{command} %*
"#;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const DEFAULT_SHELL: Shell = Shell::Bash;

#[cfg(target_os = "windows")]
const DEFAULT_SHELL: Shell = Shell::Cmd;

#[derive(Serialize)]
struct TemplateContext {
    path_env: String,
    vars: String,
    command: String,
}

pub fn wrap(
    app: &App,
    wrapper_file_name: &WrapperFileName,
    script_path: &Path,
    base_dir: &Path,
    shell: Option<Shell>,
    force: bool,
) -> Result<Status> {
    let Some(dir_info) = app.find_dir_info(None)? else {
        return_user_error!(
            "could not find environment for directory {}",
            app.cwd().display()
        );
    };

    let Some(env_info) = dir_info.make_env_info(Some(base_dir))? else {
        return_user_error!("could not get environment info");
    };

    let shell = shell.unwrap_or(DEFAULT_SHELL);

    let wrapper_template = match shell {
        Shell::Bash => BASH_WRAPPER_TEMPLATE,
        Shell::Cmd => CMD_WRAPPER_TEMPLATE,
    };

    let mut template = TinyTemplate::new();
    template.set_default_formatter(&format_unescaped);
    template.add_template("WRAPPER", wrapper_template)?;

    let path_env = String::from(
        make_path_env(&env_info.path_dirs)?
            .to_str()
            .ok_or_else(|| anyhow!("failed to generate PATH environment variable"))?,
    );

    let vars = make_vars(&env_info.vars);

    let wrapper_path = app
        .cache_dir()
        .join("bin")
        .join(wrapper_file_name.as_os_str());

    let command = make_script_command(&dir_info, script_path, Platform::Unix, shell)?;

    let s = template.render(
        "WRAPPER",
        &TemplateContext {
            path_env,
            vars,
            command,
        },
    )?;

    safe_write_file(&wrapper_path, s, force)?;
    ensure_file_executable_mode(&wrapper_path)?;
    info!("wrapper created at {}", wrapper_path.display());
    return_success!();
}

fn make_path_env(paths: &[PathBuf]) -> Result<OsString> {
    let mut new_paths = paths.to_owned();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    new_paths.push(PathBuf::from("$PATH"));

    #[cfg(target_os = "windows")]
    new_paths.push(PathBuf::from("%PATH%"));

    let mut s = OsString::new();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    s.push("PATH=");

    #[cfg(target_os = "windows")]
    s.push("set PATH=");

    s.push(join_paths(new_paths)?);
    Ok(s)
}

fn make_vars(vars: &[(String, String)]) -> String {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn inner(vars: &[(String, String)]) -> String {
        vars.iter()
            .map(|(k, v)| format!("{k}={v} \\\n"))
            .collect::<String>()
    }

    #[cfg(target_os = "windows")]
    fn inner(vars: &[(String, String)]) -> String {
        vars.iter()
            .map(|(k, v)| format!("set {k}={v}\n"))
            .collect::<String>()
    }

    inner(vars)
}

fn make_script_command(
    dir_info: &DirInfo,
    script_path: &Path,
    platform: Platform,
    shell: Shell,
) -> Result<String> {
    if let Some(s) = dir_info.make_script_command(script_path, platform, shell)? {
        return Ok(String::from(
            s.to_str()
                .ok_or_else(|| anyhow!("cannot convert OS string"))?,
        ));
    }

    if !is_executable_file(script_path)? {
        bail!(
            "cannot wrap script {} since it is not executable",
            script_path.display()
        );
    }

    Ok(String::from(
        script_path
            .to_str()
            .ok_or_else(|| anyhow!("cannot convert path"))?,
    ))
}
