use crate::app::App;
use crate::env_info::get_env_info;
use crate::error::Result;
use crate::util::safe_write_file;
use serde::Serialize;
use std::fs::{metadata, set_permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const PYTHON_EXECUTABLE_NAME: &'static str = "python3";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const WRAPPER_TEMPLATE: &'static str = r#"#!/bin/bash
set -euo pipefail
{path_env} \
PYTHONPATH={base_dir} \
exec {python_executable_name} {script_path} "$@""#;

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
    script_path: &PathBuf,
    base_dir: &PathBuf,
) -> Result<()> {
    let env_info = get_env_info(app, None)?;
    let path_env = format!(
        "PATH={}:$PATH",
        env_info.full_python_dir.join("bin").display()
    );

    let mut template = TinyTemplate::new();
    template.add_template("WRAPPER", WRAPPER_TEMPLATE)?;

    safe_write_file(
        wrapper_path,
        template.render(
            "WRAPPER",
            &Context {
                path_env: path_env,
                base_dir: base_dir.to_path_buf(),
                python_executable_name: PathBuf::from(PYTHON_EXECUTABLE_NAME),
                script_path: script_path.to_path_buf(),
            },
        )?,
        false,
    )?;

    let mut permissions = metadata(wrapper_path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o100);
    set_permissions(wrapper_path, permissions)?;

    Ok(())
}
