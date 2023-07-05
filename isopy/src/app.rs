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
use crate::constants::ENV_FILE_NAME;
use crate::plugin::Plugin;
use crate::serialization::{EnvRec, PackageDirRec};
use crate::unpack::unpack_file;
use anyhow::{bail, Result};
use isopy_lib::Descriptor;
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

    pub async fn download_asset(
        &self,
        plugin: &Plugin,
        descriptor: &dyn Descriptor,
        shared_dir: &Path,
    ) -> Result<PathBuf> {
        plugin.download_asset(descriptor, shared_dir).await
    }

    pub async fn add_package(&self, plugin: &Plugin, descriptor: &dyn Descriptor) -> Result<()> {
        let project_dir = self.cwd.clone();
        let mut package_dirs = Vec::new();

        let (dir_info, env_config_path) = if let Some(dir_info) = self.repo.get(&project_dir)? {
            let env_config_path = dir_info.data_dir().join(ENV_FILE_NAME);
            let env_rec = read_yaml_file::<EnvRec>(&env_config_path)?;
            if env_rec.project_dir != project_dir {
                bail!(
                    "environment file {} does not refer to project directory {}",
                    env_config_path.display(),
                    project_dir.display()
                );
            }

            package_dirs.extend(env_rec.package_dirs);
            (dir_info, env_config_path)
        } else {
            let Some(dir_info) = self.repo.init(&project_dir)? else {
                bail!(
                    "could not initialize environment for directory {}",
                    project_dir.display()
                )
            };

            let env_config_path = dir_info.data_dir().join(ENV_FILE_NAME);
            (dir_info, env_config_path)
        };

        if package_dirs.iter().any(|p| p.id == plugin.prefix()) {
            bail!(
                "environment already has a package with ID {} configured",
                plugin.prefix()
            );
        }

        let asset_path = self
            .download_asset(plugin, descriptor, self.repo.shared_dir())
            .await?;

        unpack_file(descriptor, &asset_path, dir_info.data_dir())?;

        package_dirs.push(PackageDirRec {
            id: String::from(plugin.prefix()),
            properties: descriptor.get_env_config_value()?,
        });

        safe_write_file(
            &env_config_path,
            serde_yaml::to_string(&EnvRec {
                project_dir,
                package_dirs,
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
