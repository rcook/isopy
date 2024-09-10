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
use crate::fs::default_config_dir;
use crate::serialization::{Env, Package, Project};
use crate::shell::IsopyEnv;
use crate::tng::{Moniker, PluginManager};
use anyhow::{bail, Result};
use isopy_lib::tng::Version;
use joat_repo::{DirInfo, Link, LinkId, Repo, RepoResult};
use joatmon::{read_yaml_file, safe_write_file, FileReadError, HasOtherError, YamlError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) struct App {
    config_dir: PathBuf,
    offline: bool,
    cwd: PathBuf,
    cache_dir: PathBuf,
    repo: Repo,
    project_config_path: PathBuf,
    plugin_manager: PluginManager,
}

impl App {
    pub(crate) fn new(offline: bool, cwd: PathBuf, cache_dir: &Path, repo: Repo) -> Result<Self> {
        let project_config_path = cwd.join(&*PROJECT_CONFIG_FILE_NAME);
        let plugin_manager = PluginManager::new();
        Ok(Self {
            config_dir: default_config_dir()?,
            offline,
            cwd,
            cache_dir: cache_dir.to_path_buf(),
            repo,
            project_config_path,
            plugin_manager,
        })
    }

    pub(crate) const fn offline(&self) -> bool {
        self.offline
    }

    pub(crate) fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub(crate) fn cwd(&self) -> &Path {
        &self.cwd
    }

    pub(crate) fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub(crate) const fn repo(&self) -> &Repo {
        &self.repo
    }

    pub(crate) fn has_project_config_file(&self) -> bool {
        self.project_config_path.is_file()
    }

    pub(crate) fn read_project_config(&self) -> Result<Project> {
        Ok(read_yaml_file(&self.project_config_path)?)
    }

    pub(crate) fn write_project_config(&self, project: &Project, overwrite: bool) -> Result<()> {
        safe_write_file(
            &self.project_config_path,
            serde_yaml::to_string(project)?,
            overwrite,
        )?;
        Ok(())
    }

    pub(crate) async fn install_package(&self, moniker: &Moniker, version: &Version) -> Result<()> {
        let project_dir = &self.cwd;

        let (dir_info, mut packages) = if let Some(dir_info) = self.repo.get(&project_dir)? {
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
            let Some(dir_info) = self.repo.init(&project_dir)? else {
                bail!(
                    "Could not initialize environment for directory {}",
                    project_dir.display()
                )
            };

            (dir_info, Vec::new())
        };

        if packages.iter().any(|p| p.id == moniker.as_str()) {
            bail!(
                "Environment already has a package with ID {} configured",
                moniker.as_str()
            );
        }

        let package_manager = self
            .plugin_manager
            .new_package_manager(&moniker, &self.config_dir);

        let bin_subdir = Path::new(moniker.as_str());

        package_manager
            .on_before_install(dir_info.data_dir(), bin_subdir)
            .await?;

        let output_path = dir_info.data_dir().join(bin_subdir);
        let package_info = package_manager
            .install_package(&version, &None, &output_path)
            .await?;

        package_manager
            .on_after_install(dir_info.data_dir(), bin_subdir)
            .await?;

        packages.push(Package {
            id: String::from(moniker.as_str()),
            props: package_info.get_env_props(bin_subdir)?,
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

    pub(crate) fn find_link(&self, link_id: &LinkId) -> RepoResult<Option<Link>> {
        // THIS IS A TEMPORARY HACK!
        // joat-repo-rs needs a method to get a DirInfo given a link ID or something
        for link in self.repo.list_links()? {
            if link.link_id() == link_id {
                return Ok(Some(link));
            }
        }

        Ok(None)
    }

    pub(crate) fn get_dir_info(&self, project_dir: &Path) -> Result<Option<DirInfo>> {
        Ok(self.repo.get(project_dir)?)
    }

    pub(crate) fn remove_project_env(&self, project_dir: &Path) -> Result<bool> {
        Ok(self.repo.remove(project_dir)?)
    }

    pub(crate) fn find_dir_info(&self, isopy_env: Option<&IsopyEnv>) -> Result<Option<DirInfo>> {
        if let Some(isopy_env) = isopy_env {
            let Some(link) = self.find_link(isopy_env.link_id())? else {
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

        // Well, this is painful...
        let dir_info = match self.repo.get(link.project_dir()) {
            Ok(dir_info) => dir_info,
            Err(e) if e.is_other() => {
                let Some(e0) = e.downcast_other_ref::<YamlError>() else {
                    bail!(e);
                };

                let Some(e1) = e0.downcast_other_ref::<FileReadError>() else {
                    bail!(e);
                };

                if !e1.is_not_found() {
                    bail!(e);
                }

                None
            }
            Err(e) => bail!(e),
        };

        Ok(dir_info)
    }

    pub(crate) fn plugin_manager(&self) -> &PluginManager {
        &self.plugin_manager
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
