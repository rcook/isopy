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
use crate::asset_filter::AssetFilter;
use crate::constants::{
    INDEX_FILE_NAME, PLUGIN_NAME, PROJECT_CONFIG_FILE_NAME, RELEASES_FILE_NAME, RELEASES_URL,
    REPOSITORIES_FILE_NAME,
};
use crate::python_descriptor::PythonDescriptor;
use crate::serialization::{EnvConfigRec, ProjectConfigRec};
use crate::serialization::{IndexRec, PackageRec, RepositoriesRec, RepositoryRec};
use crate::{
    download_asset, get_asset, Asset, AssetMeta, GitHubRepository, LocalRepository, Repository,
    RepositoryInfo, RepositoryName,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use isopy_lib::{
    dir_url, download_stream, Descriptor, DownloadAssetResult, EnvInfo, GetPackageInfosResult,
    LastModified, PackageInfo, ParseDescriptorError, ParseDescriptorResult, Product,
    ReadEnvConfigError, ReadEnvConfigResult, ReadProjectConfigFileError,
    ReadProjectConfigFileResult,
};
use joatmon::label_file_name;
use joatmon::read_yaml_file;
use joatmon::{read_json_file, safe_write_file};
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use url::Url;

pub struct Python;

impl Python {
    async fn download_python(
        &self,
        descriptor: &PythonDescriptor,
        shared_dir: &Path,
    ) -> DownloadAssetResult<PathBuf> {
        let assets = self.read_assets(shared_dir)?;
        let asset = get_asset(&assets, descriptor)?;
        let repositories = self.read_repositories(shared_dir)?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;
        let asset_path = download_asset(repository, asset, shared_dir).await?;
        Ok(asset_path)
    }

    fn read_assets(&self, shared_dir: &Path) -> Result<Vec<Asset>> {
        let index_json_path = shared_dir.join(RELEASES_FILE_NAME);
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

    fn read_repositories(&self, shared_dir: &Path) -> Result<Vec<RepositoryInfo>> {
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

        let repositories_yaml_path = shared_dir.join(REPOSITORIES_FILE_NAME);
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
            .map(make_repository);
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

    #[allow(unused)]
    async fn show_python_index(&self, shared_dir: &Path) -> Result<Vec<PackageInfo>> {
        self.update_index_if_necessary(shared_dir).await?;
        self.show_available_downloads(shared_dir)
    }

    async fn update_index_if_necessary(&self, shared_dir: &Path) -> Result<()> {
        let repositories = self.read_repositories(shared_dir)?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;

        let releases_path = self.releases_path(&repository.name, shared_dir);
        let current_last_modified = if releases_path.is_file() {
            self.read_index_last_modified(&repository.name, shared_dir)?
        } else {
            None
        };

        if let Some(mut response) = repository
            .repository
            .get_latest_index(&current_last_modified)
            .await?
        {
            download_stream("release index", &mut response, &releases_path).await?;
            if let Some(last_modified) = response.last_modified() {
                self.write_index_last_modified(&repository.name, last_modified, shared_dir)?;
            }
        }

        Ok(())
    }

    fn show_available_downloads(&self, shared_dir: &Path) -> Result<Vec<PackageInfo>> {
        fn compare_by_version_and_tag(a: &Asset, b: &Asset) -> Ordering {
            match a.meta.version.cmp(&b.meta.version) {
                Ordering::Equal => a.tag.cmp(&b.tag),
                result => result,
            }
        }

        let mut assets = self.read_assets(shared_dir)?;
        assets.sort_by(|a, b| compare_by_version_and_tag(b, a));

        Ok(AssetFilter::default_for_platform()
            .filter(assets.iter())
            .into_iter()
            .map(|asset| PackageInfo {
                descriptor: Box::new(PythonDescriptor {
                    version: asset.meta.version.clone(),
                    tag: Some(asset.tag.clone()),
                }),
                file_name: PathBuf::from(asset.name.clone()),
            })
            .collect::<Vec<_>>())
    }

    fn releases_path(&self, repository_name: &RepositoryName, shared_dir: &Path) -> PathBuf {
        if repository_name.is_default() {
            shared_dir.join(RELEASES_FILE_NAME)
        } else {
            label_file_name(
                &shared_dir.join(RELEASES_FILE_NAME),
                repository_name.as_str(),
            )
            .expect("must be valid")
        }
    }

    fn read_index_last_modified(
        &self,
        repository_name: &RepositoryName,
        shared_dir: &Path,
    ) -> Result<Option<LastModified>> {
        let index_path = self.index_path(repository_name, shared_dir);
        Ok(if index_path.is_file() {
            Some(read_yaml_file::<IndexRec>(&index_path)?.last_modified)
        } else {
            None
        })
    }

    fn write_index_last_modified(
        &self,
        repository_name: &RepositoryName,
        last_modified: &LastModified,
        shared_dir: &Path,
    ) -> Result<()> {
        let index_yaml_path = self.index_path(repository_name, shared_dir);
        safe_write_file(
            &index_yaml_path,
            serde_yaml::to_string(&IndexRec {
                last_modified: last_modified.clone(),
            })?,
            true,
        )?;
        Ok(())
    }

    fn index_path(&self, repository_name: &RepositoryName, shared_dir: &Path) -> PathBuf {
        if repository_name.is_default() {
            shared_dir.join(INDEX_FILE_NAME)
        } else {
            label_file_name(&shared_dir.join(INDEX_FILE_NAME), repository_name.as_str())
                .expect("must be valid")
        }
    }
}

impl Default for Python {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Product for Python {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn url(&self) -> &Url {
        &RELEASES_URL
    }

    fn project_config_file_name(&self) -> &Path {
        &PROJECT_CONFIG_FILE_NAME
    }

    fn read_project_config_file(
        &self,
        path: &Path,
    ) -> ReadProjectConfigFileResult<Box<dyn Descriptor>> {
        let project_config_rec = read_yaml_file::<ProjectConfigRec>(path)
            .map_err(|e| ReadProjectConfigFileError::Other(anyhow!(e)))?;

        Ok(Box::new(PythonDescriptor {
            version: project_config_rec.version,
            tag: project_config_rec.tag,
        }))
    }

    fn parse_descriptor(&self, s: &str) -> ParseDescriptorResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<PythonDescriptor>()
                .map_err(|e| ParseDescriptorError::Other(anyhow!(e)))?,
        ))
    }

    async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        shared_dir: &Path,
    ) -> DownloadAssetResult<PathBuf> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<PythonDescriptor>()
            .expect("must be PythonDescriptor");
        self.download_python(descriptor, shared_dir).await
    }

    fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> ReadEnvConfigResult<EnvInfo> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfigRec) -> Vec<PathBuf> {
            vec![data_dir.join(&env_config_rec.dir).join("bin")]
        }

        #[cfg(target_os = "windows")]
        fn make_path_dirs(data_dir: &Path, env_config_rec: &EnvConfigRec) -> Vec<PathBuf> {
            vec![
                data_dir.join(&env_config_rec.dir).join("bin"),
                data_dir.join(&env_config_rec.dir).join("Scripts"),
            ]
        }

        let env_config_rec = serde_json::from_value::<EnvConfigRec>(properties.clone())
            .map_err(|e| ReadEnvConfigError::Other(anyhow!(e)))?;

        Ok(EnvInfo {
            path_dirs: make_path_dirs(data_dir, &env_config_rec),
            envs: vec![],
        })
    }

    async fn get_package_infos(
        &self,
        _shared_dir: &Path,
    ) -> GetPackageInfosResult<Vec<PackageInfo>> {
        //todo!();
        Ok(Vec::new())
    }
}
