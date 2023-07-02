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
use crate::adoptium::AdoptiumIndexManager;
use crate::api::python_standalone_builds::LastModified;
use crate::asset::{download_asset, get_asset};
use crate::checksum::verify_sha256_file_checksum;
use crate::constants::{
    ADOPTIUM_INDEX_FILE_NAME, ADOPTIUM_SERVER_URL, ENV_FILE_NAME, INDEX_FILE_NAME,
    RELEASES_FILE_NAME, RELEASES_URL, REPOSITORIES_FILE_NAME,
};
use crate::python::{Asset, AssetMeta};
use crate::repository::{GitHubRepository, LocalRepository, Repository};
use crate::repository_name::RepositoryName;
use crate::serialization::{EnvRec, PythonEnvRec};
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use crate::unpack::{unpack_file, NoOpUnpackPathTransform};
use crate::url::dir_url;
use anyhow::{bail, Result};
use isopy_openjdk::OpenJdkProductDescriptor;
use isopy_python::PythonProductDescriptor;
use joat_repo::{DirInfo, Link, LinkId, Repo, RepoResult};
use joatmon::{label_file_name, read_json_file, read_yaml_file, safe_write_file};
use log::info;
use std::collections::HashMap;
use std::fs::remove_file;
use std::path::{Path, PathBuf};

pub struct RepositoryInfo {
    pub name: RepositoryName,
    pub repository: Box<dyn Repository>,
}

#[derive(Debug)]
pub struct App {
    pub cwd: PathBuf,
    pub repo: Repo,
}

impl App {
    pub const fn new(cwd: PathBuf, repo: Repo) -> Self {
        Self { cwd, repo }
    }

    pub async fn download_python(
        &self,
        product_descriptor: &PythonProductDescriptor,
    ) -> Result<()> {
        let assets = self.read_assets()?;
        let asset = get_asset(
            &assets,
            &PythonProductDescriptor {
                version: product_descriptor.version.clone(),
                tag: product_descriptor.tag.clone(),
            },
        )?;
        download_asset(self, asset).await?;
        Ok(())
    }

    pub async fn download_openjdk(
        &self,
        product_descriptor: &OpenJdkProductDescriptor,
    ) -> Result<PathBuf> {
        let manager = AdoptiumIndexManager::new(
            &ADOPTIUM_SERVER_URL,
            &self.repo.shared_dir().join(ADOPTIUM_INDEX_FILE_NAME),
        );

        let versions = manager.read_versions().await?;
        let Some(version) = versions
            .iter()
            .find(|x| x.openjdk_version == product_descriptor.version) else {
            bail!("no version matching {}", product_descriptor.version);
        };

        let output_path = self.repo.shared_dir().join(&version.file_name);
        if output_path.exists() {
            info!("{} already downloaded", version.file_name.display());
            return Ok(output_path);
        }

        manager.download_asset(&version.url, &output_path).await?;

        let is_valid = verify_sha256_file_checksum(&version.checksum, &output_path)?;
        if !is_valid {
            remove_file(&output_path)?;
            bail!(
                "SHA256 checksum validation failed on {}",
                output_path.display()
            );
        }

        info!(
            "SHA256 checksum validation succeeded on {}",
            output_path.display()
        );

        Ok(output_path)
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

    pub fn make_asset_path(&self, asset: &Asset) -> PathBuf {
        self.repo.shared_dir().join(&asset.name)
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

    pub async fn init_project(&self, product_descriptor: &PythonProductDescriptor) -> Result<()> {
        let assets = self.read_assets()?;
        let asset = get_asset(&assets, product_descriptor)?;

        let mut asset_path = self.make_asset_path(asset);
        if !asset_path.is_file() {
            asset_path = download_asset(self, asset).await?;
        }

        let Some(dir_info) = self.repo.init(&self.cwd)? else {
            bail!(
                "Could not initialize metadirectory for directory {}",
                self.cwd.display()
            )
        };

        unpack_file::<NoOpUnpackPathTransform>(&asset_path, dir_info.data_dir())?;

        safe_write_file(
            &dir_info.data_dir().join(ENV_FILE_NAME),
            serde_yaml::to_string(&EnvRec {
                config_path: self.cwd.clone(),
                python: Some(PythonEnvRec {
                    dir: PathBuf::from("python"),
                    version: asset.meta.version.clone(),
                    tag: asset.tag.clone(),
                }),
                openjdk: None,
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
