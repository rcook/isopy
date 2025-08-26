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
use crate::constants::PROJECT_CONFIG_FILE_NAME;
use crate::dir_info_ext::DirInfoExt;
use crate::formats::read_yaml_file;
use crate::fs::safe_write_file;
use crate::moniker::Moniker;
use crate::package_id::PackageId;
use crate::plugin_manager::PluginManager;
use crate::repo::{DirInfo, Link, LinkId, Repo};
use crate::serialization::{Env, EnvPackage, Project};
use crate::shell::IsopyEnv;
use anyhow::{Result, bail};
use isopy_lib::{
    EnvInfo, GetPackageOptions, InstallPackageOptions, PackageInfo, Platform, Shell, TagFilter,
    Version,
};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};

pub(crate) struct App {
    pub(crate) config_dir: PathBuf,
    pub(crate) cwd: PathBuf,
    pub(crate) repo: Repo,
    pub(crate) plugin_manager: PluginManager,
    pub(crate) show_progress: bool,
    pub config_value_path: PathBuf,
    project_config_path: PathBuf,
}

impl App {
    pub(crate) fn new(cwd: &Path, config_dir: &Path, repo: Repo, show_progress: bool) -> Self {
        Self {
            config_dir: config_dir.to_path_buf(),
            cwd: cwd.to_path_buf(),
            repo,
            plugin_manager: PluginManager::new(),
            show_progress,
            config_value_path: config_dir.join("config-values.yaml"),
            project_config_path: cwd.join(PROJECT_CONFIG_FILE_NAME),
        }
    }

    pub(crate) fn get_config_values(&self) -> Result<HashMap<String, String>> {
        if self.config_value_path.is_file() {
            Ok(read_yaml_file::<HashMap<String, String>>(
                &self.config_value_path,
            )?)
        } else {
            Ok(HashMap::new())
        }
    }

    pub(crate) fn get_config_value(&self, name: &str) -> Result<Option<String>> {
        if !self.config_value_path.is_file() {
            return Ok(None);
        }

        let config_values = read_yaml_file::<HashMap<String, String>>(&self.config_value_path)?;
        Ok(config_values.get(name).cloned())
    }

    pub(crate) fn set_config_value(&self, name: &str, value: &str) -> Result<()> {
        let mut config_values = if self.config_value_path.is_file() {
            read_yaml_file::<HashMap<String, String>>(&self.config_value_path)?
        } else {
            HashMap::new()
        };
        _ = config_values.insert(name.to_owned(), value.to_owned());
        let f = File::create(&self.config_value_path)?;
        serde_yaml::to_writer(f, &config_values)?;
        Ok(())
    }

    pub(crate) fn delete_config_value(&self, name: &str) -> Result<()> {
        if self.config_value_path.is_file() {
            let mut config_values =
                read_yaml_file::<HashMap<String, String>>(&self.config_value_path)?;
            if config_values.remove(name).is_some() {
                let f = File::create(&self.config_value_path)?;
                serde_yaml::to_writer(f, &config_values)?;
            }
        }
        Ok(())
    }

    pub(crate) fn has_project_config_file(&self) -> bool {
        self.project_config_path.is_file()
    }

    pub(crate) fn read_project_config(&self) -> Result<Project> {
        read_yaml_file(&self.project_config_path)
    }

    pub(crate) fn write_project_config(&self, project: &Project, overwrite: bool) -> Result<()> {
        safe_write_file(
            &self.project_config_path,
            serde_yaml::to_string(project)?,
            overwrite,
        )?;
        Ok(())
    }

    pub(crate) async fn get_package(
        &self,
        moniker: &Moniker,
        version: &Version,
        options: &GetPackageOptions,
    ) -> Result<Option<PackageInfo>> {
        self.plugin_manager
            .new_package_manager(moniker, &self.config_dir)
            .get_package(version, &TagFilter::default(), options)
            .await
    }

    pub(crate) async fn install_package(
        &self,
        moniker: &Moniker,
        version: &Version,
        options: &InstallPackageOptions,
    ) -> Result<()> {
        let project_dir = &self.cwd;

        let (dir_info, mut packages) = if let Some(dir_info) = self.repo.get(project_dir)? {
            let env = dir_info.read_env_config()?;
            if env.project_dir != *project_dir {
                bail!(
                    "Environment directory {} does not correspond to project directory {}",
                    dir_info.data_dir().display(),
                    project_dir.display()
                );
            }

            (dir_info, env.packages)
        } else {
            let Some(dir_info) = self.repo.init(project_dir)? else {
                bail!(
                    "Could not initialize environment for directory {}",
                    project_dir.display()
                );
            };

            (dir_info, Vec::new())
        };

        if packages.iter().any(|p| &p.package_id.moniker == moniker) {
            bail!("Environment already has a package for package manager {moniker} configured");
        }

        let package_manager = self
            .plugin_manager
            .new_package_manager(moniker, &self.config_dir);

        let dir_name = Path::new(moniker.as_str());
        let output_path = dir_info.data_dir().join(dir_name);

        let package = match package_manager
            .install_package(version, &TagFilter::default(), &output_path, options)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                _ = self.remove_project_env(project_dir);
                bail!(e)
            }
        };

        packages.push(EnvPackage {
            package_id: PackageId::new(moniker, package.version()),
            dir: dir_name.to_path_buf(),
            url: package.url().to_owned(),
        });

        dir_info.write_env_config(
            &Env {
                project_dir: project_dir.clone(),
                packages,
            },
            true,
        )?;

        Ok(())
    }

    pub(crate) fn find_link(&self, link_id: &LinkId) -> Result<Option<Link>> {
        // THIS IS A TEMPORARY HACK!
        // isopy-repo needs a method to get a DirInfo given a link ID or something
        for link in self.repo.list_links()? {
            if link.link_id() == link_id {
                return Ok(Some(link));
            }
        }

        Ok(None)
    }

    pub(crate) fn get_dir_info(&self, project_dir: &Path) -> Result<Option<DirInfo>> {
        self.repo.get(project_dir)
    }

    pub(crate) fn remove_project_env(&self, project_dir: &Path) -> Result<bool> {
        self.repo.remove(project_dir)
    }

    pub(crate) fn find_dir_info(&self, isopy_env: Option<&IsopyEnv>) -> Result<Option<DirInfo>> {
        if let Some(isopy_env) = isopy_env {
            let Some(link) = self.find_link(&isopy_env.link_id)? else {
                return Ok(None);
            };

            let Some(dir_info) = self.repo.get(link.project_dir())? else {
                return Ok(None);
            };

            return Ok(Some(dir_info));
        }

        let Some(link) = self.find_link_for_dir(&self.cwd)? else {
            return Ok(None);
        };

        self.repo.get(link.project_dir())
    }

    pub(crate) fn make_env_info(&self, data_dir: &Path, package: &EnvPackage) -> EnvInfo {
        self.plugin_manager
            .get_plugin(&package.package_id.moniker)
            .make_env_info(&data_dir.join(&package.dir))
    }

    pub(crate) fn make_script_command(
        &self,
        package: &EnvPackage,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>> {
        let plugin = self.plugin_manager.get_plugin(&package.package_id.moniker);
        plugin.make_script_command(script_path, platform, shell)
    }

    fn find_link_for_dir(&self, dir: &Path) -> Result<Option<Link>> {
        let mut map = self
            .repo
            .list_links()?
            .into_iter()
            .map(|x| (x.project_dir().to_path_buf(), x))
            .collect::<HashMap<_, _>>();

        let mut d = dir;
        loop {
            if let Some(link) = map.remove(d) {
                return Ok(Some(link));
            }

            if let Some(p) = d.parent() {
                d = p;
            } else {
                return Ok(None);
            }
        }
    }
}
