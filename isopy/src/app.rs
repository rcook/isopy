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
use crate::asset::{download_asset, get_asset};
use crate::constants::{
    ENV_FILE_NAME, INDEX_FILE_NAME, OPENJDK_DESCRIPTOR_PREFIX, PYTHON_DESCRIPTOR_PREFIX,
    RELEASES_FILE_NAME, RELEASES_URL, REPOSITORIES_FILE_NAME,
};
use crate::python::{Asset, AssetMeta};
use crate::registry::{DescriptorId, ProductDescriptor, ProductInfo, ProductRegistry};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::repository_name::RepositoryName;
use crate::serialization::{EnvRec, PackageDirRec};
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use crate::unpack::unpack_file;
use crate::url::dir_url;
use anyhow::{bail, Result};
use isopy_lib::{Descriptor, LastModified};
use isopy_openjdk::OpenJdk;
use isopy_python::{Python, PythonDescriptor};
use joat_repo::{DirInfo, Link, LinkId, Repo, RepoResult};
use joatmon::{label_file_name, read_json_file, read_yaml_file, safe_write_file};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct RepositoryInfo {
    pub name: RepositoryName,
    pub repository: Box<dyn Repository>,
}

pub struct App {
    pub cwd: PathBuf,
    pub repo: Repo,
    pub registry: ProductRegistry,
}

impl App {
    pub fn new(cwd: PathBuf, repo: Repo) -> Self {
        let registry = ProductRegistry::new(vec![
            ProductInfo {
                prefix: String::from(PYTHON_DESCRIPTOR_PREFIX),
                product: Box::<Python>::default(),
            },
            ProductInfo {
                prefix: String::from(OPENJDK_DESCRIPTOR_PREFIX),
                product: Box::<OpenJdk>::default(),
            },
        ]);
        Self {
            cwd,
            repo,
            registry,
        }
    }

    pub async fn download_asset(
        &self,
        descriptor_id: &DescriptorId,
        shared_dir: &Path,
    ) -> Result<PathBuf> {
        let descriptor_info = self.registry.to_descriptor_info(descriptor_id)?;
        let product_descriptor = descriptor_info.to_product_descriptor()?;

        Ok(match &product_descriptor {
            ProductDescriptor::Python(d) => self.download_python(d, shared_dir).await?,
            ProductDescriptor::OpenJdk(_) => {
                descriptor_info
                    .product_info
                    .product
                    .download_asset(descriptor_info.descriptor.as_ref(), shared_dir)
                    .await?
            }
        })
    }

    pub async fn download_python(
        &self,
        descriptor: &PythonDescriptor,
        shared_dir: &Path,
    ) -> Result<PathBuf> {
        let assets = self.read_assets()?;
        let asset = get_asset(&assets, descriptor)?;
        let asset_path = download_asset(self, asset, shared_dir).await?;
        Ok(asset_path)
    }

    pub fn read_repositories(&self) -> Result<Vec<RepositoryInfo>> {
        let repositories_yaml_path = self.repo.shared_dir().join(REPOSITORIES_FILE_NAME);
        let repositories_rec = if repositories_yaml_path.is_file() {
            read_yaml_file::<RepositoriesRec>(&repositories_yaml_path)?
        } else {
            let repositories_rec = RepositoriesRec {
                repositories: vec![
                    RepositoryRec::GitHub {
                        name: RepositoryName::Default,
                        url: dir_url(&RELEASES_URL),
                        enabled: true,
                    },
                    RepositoryRec::Local {
                        name: RepositoryName::Example,
                        dir: PathBuf::from("/path/to/local/repository"),
                        enabled: false,
                    },
                ],
            };
            safe_write_file(
                &repositories_yaml_path,
                serde_yaml::to_string(&repositories_rec)?,
                false,
            )?;
            repositories_rec
        };

        let all_repositories = repositories_rec
            .repositories
            .into_iter()
            .map(Self::make_repository);
        let enabled_repositories = all_repositories
            .into_iter()
            .filter(|x| x.1)
            .map(|x| RepositoryInfo {
                name: x.0,
                repository: x.2,
            })
            .collect::<Vec<_>>();
        Ok(enabled_repositories)
    }

    pub fn read_index_last_modified(
        &self,
        repository_name: &RepositoryName,
    ) -> Result<Option<LastModified>> {
        let index_path = self.index_path(repository_name);
        Ok(if index_path.is_file() {
            Some(read_yaml_file::<IndexRec>(&index_path)?.last_modified)
        } else {
            None
        })
    }

    pub fn write_index_last_modified(
        &self,
        repository_name: &RepositoryName,
        last_modified: &LastModified,
    ) -> Result<()> {
        let index_yaml_path = self.index_path(repository_name);
        safe_write_file(
            &index_yaml_path,
            serde_yaml::to_string(&IndexRec {
                last_modified: last_modified.clone(),
            })?,
            true,
        )?;
        Ok(())
    }

    pub fn releases_path(&self, repository_name: &RepositoryName) -> PathBuf {
        if repository_name.is_default() {
            self.repo.shared_dir().join(RELEASES_FILE_NAME)
        } else {
            label_file_name(
                &self.repo.shared_dir().join(RELEASES_FILE_NAME),
                repository_name.as_str(),
            )
            .expect("must be valid")
        }
    }

    pub fn read_assets(&self) -> Result<Vec<Asset>> {
        let index_json_path = self.repo.shared_dir().join(RELEASES_FILE_NAME);
        let package_recs = read_json_file::<Vec<PackageRec>>(&index_json_path)?;

        let mut assets = Vec::new();
        for package_rec in package_recs {
            for asset_rec in package_rec.assets {
                if !AssetMeta::definitely_not_an_asset_name(&asset_rec.name) {
                    let meta = asset_rec.name.parse::<AssetMeta>()?;
                    assets.push(Asset {
                        name: asset_rec.name,
                        tag: package_rec.tag.clone(),
                        url: asset_rec.url,
                        size: asset_rec.size,
                        meta,
                    });
                }
            }
        }
        Ok(assets)
    }

    pub async fn init_project(&self, descriptor_id: &DescriptorId) -> Result<()> {
        let descriptor_info = self.registry.to_descriptor_info(descriptor_id)?;
        let product_descriptor = descriptor_info.to_product_descriptor()?;

        let d: &dyn Descriptor = match &product_descriptor {
            ProductDescriptor::Python(d) => d,
            ProductDescriptor::OpenJdk(d) => d,
        };

        let asset_path = self
            .download_asset(descriptor_id, self.repo.shared_dir())
            .await?;

        let Some(dir_info) = self.repo.init(&self.cwd)? else {
            bail!(
                "Could not initialize metadirectory for directory {}",
                self.cwd.display()
            )
        };

        unpack_file(d, &asset_path, dir_info.data_dir())?;

        safe_write_file(
            &dir_info.data_dir().join(ENV_FILE_NAME),
            serde_yaml::to_string(&EnvRec {
                config_path: self.cwd.clone(),
                python: None,
                openjdk: None,
                package_dirs: vec![PackageDirRec {
                    id: descriptor_info.product_info.prefix.clone(),
                    properties: d.get_config_value()?,
                }],
            })?,
            false,
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

    fn make_repository(rec: RepositoryRec) -> (RepositoryName, bool, Box<dyn Repository>) {
        match rec {
            RepositoryRec::GitHub { name, url, enabled } => {
                (name, enabled, Box::new(GitHubRepository::new(&url)))
            }
            RepositoryRec::Local { name, dir, enabled } => {
                (name, enabled, Box::new(LocalRepository::new(dir)))
            }
        }
    }

    fn index_path(&self, repository_name: &RepositoryName) -> PathBuf {
        if repository_name.is_default() {
            self.repo.shared_dir().join(INDEX_FILE_NAME)
        } else {
            label_file_name(
                &self.repo.shared_dir().join(INDEX_FILE_NAME),
                repository_name.as_str(),
            )
            .expect("must be valid")
        }
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
