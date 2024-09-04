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
use crate::plugin_host::PluginHost;
use crate::serialization::{EnvRec, PackageRec, ProjectRec};
use crate::shell::IsopyEnv;
use crate::tng::App as AppTNG;
use crate::unpack::unpack_file;
use anyhow::{bail, Result};
use isopy_lib::{Descriptor, Package, PluginFactory};
use joat_repo::{DirInfo, Link, LinkId, Repo, RepoResult};
use joatmon::{read_yaml_file, safe_write_file, FileReadError, HasOtherError, YamlError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct App {
    offline: bool,
    cwd: PathBuf,
    cache_dir: PathBuf,
    repo: Repo,
    project_config_path: PathBuf,
    app_tng: AppTNG,
}

impl App {
    pub fn new(offline: bool, cwd: PathBuf, cache_dir: &Path, repo: Repo) -> Result<Self> {
        let project_config_path = cwd.join(&*PROJECT_CONFIG_FILE_NAME);
        let app_tng = AppTNG::new(&default_config_dir()?)?;
        Ok(Self {
            offline,
            cwd,
            cache_dir: cache_dir.to_path_buf(),
            repo,
            project_config_path,
            app_tng,
        })
    }

    pub const fn offline(&self) -> bool {
        self.offline
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub const fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn has_project_config_file(&self) -> bool {
        self.project_config_path.is_file()
    }

    pub fn read_project_config(&self) -> Result<ProjectRec> {
        Ok(read_yaml_file(&self.project_config_path)?)
    }

    pub fn write_project_config(&self, project_rec: &ProjectRec, overwrite: bool) -> Result<()> {
        safe_write_file(
            &self.project_config_path,
            serde_yaml::to_string(project_rec)?,
            overwrite,
        )?;
        Ok(())
    }

    pub async fn add_package(
        &self,
        plugin_host: &PluginHost,
        descriptor: &dyn Descriptor,
    ) -> Result<()> {
        let project_dir = self.cwd.clone();
        let mut packages = Vec::new();

        let dir_info = if let Some(dir_info) = self.repo.get(&project_dir)? {
            let env_rec = dir_info.read_env_config()?;
            if env_rec.project_dir != project_dir {
                bail!(
                    "environment directory {} does not correspond to project directory {}",
                    dir_info.data_dir().display(),
                    project_dir.display()
                );
            }

            packages.extend(env_rec.packages);
            dir_info
        } else {
            let Some(dir_info) = self.repo.init(&project_dir)? else {
                bail!(
                    "could not initialize environment for directory {}",
                    project_dir.display()
                )
            };

            dir_info
        };

        if packages.iter().any(|p| p.id == plugin_host.prefix()) {
            bail!(
                "environment already has a package with ID {} configured",
                plugin_host.prefix()
            );
        }

        let plugin_dir = self.repo.shared_dir().join(plugin_host.prefix());
        let plugin = plugin_host.make_plugin(self.offline, &plugin_dir);
        let Package {
            asset_path,
            descriptor,
        } = plugin.download_package(descriptor).await?;

        let bin_subdir = Path::new(plugin_host.prefix());

        plugin
            .on_before_install(dir_info.data_dir(), bin_subdir)
            .await?;

        unpack_file(
            descriptor.as_ref().as_ref(),
            &asset_path,
            dir_info.data_dir(),
            bin_subdir,
        )?;

        plugin
            .on_after_install(dir_info.data_dir(), bin_subdir)
            .await?;

        packages.push(PackageRec {
            id: String::from(plugin_host.prefix()),
            props: descriptor.get_env_props(bin_subdir)?,
        });

        dir_info.write_env_config(
            &EnvRec {
                project_dir,
                packages,
            },
            true,
        )?;

        Ok(())
    }

    pub fn find_link(&self, link_id: &LinkId) -> RepoResult<Option<Link>> {
        // THIS IS A TEMPORARY HACK!
        // joat-repo-rs needs a method to get a DirInfo given a link ID or something
        for link in self.repo.list_links()? {
            if link.link_id() == link_id {
                return Ok(Some(link));
            }
        }

        Ok(None)
    }

    pub fn get_dir_info(&self, project_dir: &Path) -> Result<Option<DirInfo>> {
        Ok(self.repo.get(project_dir)?)
    }

    pub fn remove_project_env(&self, project_dir: &Path) -> Result<bool> {
        Ok(self.repo.remove(project_dir)?)
    }

    pub fn find_dir_info(&self, isopy_env: Option<&IsopyEnv>) -> Result<Option<DirInfo>> {
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

    #[allow(unused)]
    pub(crate) fn app_tng(&self) -> &AppTNG {
        &self.app_tng
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
