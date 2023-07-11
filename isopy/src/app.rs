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
use crate::constants::ENV_CONFIG_FILE_NAME;
use crate::plugin_host::PluginHost;
use crate::serialization::{EnvRec, PackageRec};
use crate::unpack::unpack_file;
use anyhow::{bail, Result};
use isopy_lib::{Descriptor, Package, PluginFactory};
use joat_repo::{DirInfo, Link, LinkId, Repo, RepoResult};
use joatmon::{read_yaml_file, safe_write_file};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct App {
    pub cwd: PathBuf,
    pub repo: Repo,
}

impl App {
    pub const fn new(cwd: PathBuf, repo: Repo) -> Self {
        Self { cwd, repo }
    }

    pub async fn add_package(
        &self,
        plugin_host: &PluginHost,
        descriptor: &dyn Descriptor,
    ) -> Result<()> {
        let project_dir = self.cwd.clone();
        let mut packages = Vec::new();

        let (dir_info, env_config_path) = if let Some(dir_info) = self.repo.get(&project_dir)? {
            let env_config_path = dir_info.data_dir().join(&*ENV_CONFIG_FILE_NAME);
            let env_rec = read_yaml_file::<EnvRec>(&env_config_path)?;
            if env_rec.project_dir != project_dir {
                bail!(
                    "environment file {} does not refer to project directory {}",
                    env_config_path.display(),
                    project_dir.display()
                );
            }

            packages.extend(env_rec.packages);
            (dir_info, env_config_path)
        } else {
            let Some(dir_info) = self.repo.init(&project_dir)? else {
                bail!(
                    "could not initialize environment for directory {}",
                    project_dir.display()
                )
            };

            let env_config_path = dir_info.data_dir().join(&*ENV_CONFIG_FILE_NAME);
            (dir_info, env_config_path)
        };

        if packages.iter().any(|p| p.id == plugin_host.prefix()) {
            bail!(
                "environment already has a package with ID {} configured",
                plugin_host.prefix()
            );
        }

        let plugin_dir = self.repo.shared_dir().join(plugin_host.prefix());
        let plugin = plugin_host.make_plugin(&plugin_dir);
        let Package {
            asset_path,
            descriptor,
        } = plugin.download_package(descriptor).await?;

        unpack_file(
            descriptor.as_ref().as_ref(),
            &asset_path,
            dir_info.data_dir(),
        )?;

        packages.push(PackageRec {
            id: String::from(plugin_host.prefix()),
            props: descriptor.get_env_props()?,
        });

        safe_write_file(
            &env_config_path,
            serde_yaml::to_string(&EnvRec {
                project_dir,
                packages,
            })?,
            true,
        )?;

        Ok(())
    }

    pub fn find_dir_info(&self, cwd: &Path, isopy_env: Option<String>) -> Result<Option<DirInfo>> {
        if let Some(isopy_env_value) = isopy_env {
            // THIS IS A TEMPORARY HACK!
            // joat-repo-rs needs a method to get a DirInfo given a link ID or something
            fn find_link(repo: &Repo, link_id: &LinkId) -> RepoResult<Option<Link>> {
                for link in repo.list_links()? {
                    if link.link_id() == link_id {
                        return Ok(Some(link));
                    }
                }

                Ok(None)
            }

            let Some((_, link_id_str)) = isopy_env_value.split_once('-') else {
                return Ok(None)
            };

            let link_id = link_id_str.parse::<LinkId>()?;

            let Some(link) = find_link(&self.repo, &link_id)? else {
                return Ok(None)
            };

            let Some(dir_info) = self.repo.get(link.project_dir())? else {
                return Ok(None)
            };

            return Ok(Some(dir_info));
        }

        let Some(link) = self.find_link(cwd)? else {
            return Ok(None)
        };

        let Some(dir_info) = self.repo.get(link.project_dir())? else {
            return Ok(None)
        };

        Ok(Some(dir_info))
    }

    fn find_link(&self, dir: &Path) -> Result<Option<Link>> {
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
