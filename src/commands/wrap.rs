use crate::app::App;
use crate::object_model::Environment;
use crate::result::Result;
use crate::util::safe_write_file;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tinytemplate::TinyTemplate;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const PYTHON_EXECUTABLE_NAME: &str = "python3";

#[cfg(target_os = "windows")]
const PYTHON_EXECUTABLE_NAME: &'static str = "python";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const WRAPPER_TEMPLATE: &str = r#"#!/bin/bash
set -euo pipefail
{path_env} \
PYTHONPATH={base_dir} \
exec {python_executable_name} {script_path} "$@""#;

#[cfg(target_os = "windows")]
const WRAPPER_TEMPLATE: &'static str = r#"@echo off
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

pub fn do_wrap(
    app: &App,
    wrapper_path: &PathBuf,
    script_path: &Path,
    base_dir: &Path,
) -> Result<()> {
    let environment = Environment::infer(app, None)?;

    let mut template = TinyTemplate::new();
    template.add_template("WRAPPER", WRAPPER_TEMPLATE)?;

    safe_write_file(
        wrapper_path,
        template.render(
            "WRAPPER",
            &Context {
                path_env: make_path_env(&environment),
                base_dir: base_dir.to_path_buf(),
                python_executable_name: PathBuf::from(PYTHON_EXECUTABLE_NAME),
                script_path: script_path.to_path_buf(),
            },
        )?,
        false,
    )?;

    set_file_attributes(wrapper_path)?;

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn make_path_env(environment: &Environment) -> String {
    format!(
        "PATH={}:$PATH",
        environment.full_python_dir.join("bin").display()
    )
}

#[cfg(target_os = "windows")]
fn make_path_env(environment: &Environment) -> String {
    format!(
        "PATH={};{};%PATH%",
        environment.full_python_dir.display(),
        environment.full_python_dir.join("Scripts").display()
    )
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn set_file_attributes<P>(wrapper_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    use std::fs::{metadata, set_permissions};
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = metadata(&wrapper_path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o100);
    set_permissions(&wrapper_path, permissions)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_file_attributes<P>(_wrapper_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    Ok(())
}
