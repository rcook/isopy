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
use crate::constants::ENV_CONFIG_FILE_NAME;
use crate::serialization::Env;
use anyhow::Result;
use isopy_lib::{EnvInfo, Platform, Shell};
use joat_repo::{DirInfo, Manifest};
use joatmon::{read_yaml_file, safe_write_file};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub(crate) trait DirInfoExt {
    fn read_env_config(&self) -> Result<Env>;
    fn write_env_config(&self, env: &Env, overwrite: bool) -> Result<()>;
    fn make_env_info(&self, app: &App) -> Result<Option<EnvInfo>>;
    fn make_script_command(
        &self,
        app: &App,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>>;
}

impl DirInfoExt for DirInfo {
    fn read_env_config(&self) -> Result<Env> {
        read_env_config(self.data_dir())
    }

    fn write_env_config(&self, env: &Env, overwrite: bool) -> Result<()> {
        write_env_config(self.data_dir(), env, overwrite)
    }

    fn make_env_info(&self, app: &App) -> Result<Option<EnvInfo>> {
        make_env_info(app, self.data_dir())
    }

    fn make_script_command(
        &self,
        app: &App,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>> {
        make_script_command(app, self.data_dir(), script_path, platform, shell)
    }
}

impl DirInfoExt for Manifest {
    fn read_env_config(&self) -> Result<Env> {
        read_env_config(self.data_dir())
    }

    fn write_env_config(&self, env: &Env, overwrite: bool) -> Result<()> {
        write_env_config(self.data_dir(), env, overwrite)
    }

    fn make_env_info(&self, app: &App) -> Result<Option<EnvInfo>> {
        make_env_info(app, self.data_dir())
    }

    fn make_script_command(
        &self,
        app: &App,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>> {
        make_script_command(app, self.data_dir(), script_path, platform, shell)
    }
}

fn make_env_config_path(data_dir: &Path) -> PathBuf {
    data_dir.join(ENV_CONFIG_FILE_NAME)
}

fn read_env_config(data_dir: &Path) -> Result<Env> {
    Ok(read_yaml_file(&make_env_config_path(data_dir))?)
}

fn write_env_config(data_dir: &Path, env: &Env, overwrite: bool) -> Result<()> {
    safe_write_file(
        &make_env_config_path(data_dir),
        serde_yaml::to_string(env)?,
        overwrite,
    )?;
    Ok(())
}

fn make_env_info(app: &App, data_dir: &Path) -> Result<Option<EnvInfo>> {
    let env = read_env_config(data_dir)?;

    let mut all_env_info = EnvInfo {
        path_dirs: Vec::new(),
        vars: Vec::new(),
    };

    for package in &env.packages {
        let env_info = app.make_env_info(data_dir, package);
        all_env_info.path_dirs.extend(env_info.path_dirs);
        all_env_info.vars.extend(env_info.vars);
    }

    Ok(Some(all_env_info))
}

fn make_script_command(
    app: &App,
    data_dir: &Path,
    script_path: &Path,
    platform: Platform,
    shell: Shell,
) -> Result<Option<OsString>> {
    let env = read_env_config(data_dir)?;

    for package in &env.packages {
        let result = app.make_script_command(package, script_path, platform, shell)?;
        if result.is_some() {
            return Ok(result);
        }
    }

    Ok(None)
}
