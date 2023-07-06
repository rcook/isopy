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
use crate::asset::Asset;
use crate::asset_filter::AssetFilter;
use crate::asset_helper::{download_asset, get_asset};
use crate::asset_meta::AssetMeta;
use crate::constants::{
    ASSETS_DIR, INDEX_FILE_NAME, PLUGIN_NAME, PROJECT_CONFIG_FILE_NAME, RELEASES_FILE_NAME,
    RELEASES_URL, REPOSITORIES_FILE_NAME,
};
use crate::github::GitHubRepository;
use crate::local::LocalRepository;
use crate::python_descriptor::PythonDescriptor;
use crate::repository_info::RepositoryInfo;
use crate::repository_name::RepositoryName;
use crate::serialization::{
    EnvConfigRec, IndexRec, PackageRec, ProjectConfigRec, RepositoriesRec, RepositoryRec,
};
use crate::traits::Repository;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use isopy_lib::{
    dir_url, download_stream, other_error as isopy_lib_other_error, Descriptor, EnvInfo,
    IsopyLibResult, LastModified, Package, PluginFactory, PluginTNG, Product,
};
use joatmon::label_file_name;
use joatmon::read_yaml_file;
use joatmon::{read_json_file, safe_write_file};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;

pub struct PythonPluginFactory;

impl Default for PythonPluginFactory {
    fn default() -> Self {
        Self
    }
}

impl PluginFactory for PythonPluginFactory {
    fn make_plugin(&self, dir: &Path) -> Box<dyn PluginTNG> {
        Box::new(Python2::new(dir))
    }
}

pub struct Python2 {
    dir: PathBuf,
    python: Python,
}

impl Python2 {
    fn new(dir: &Path) -> Self {
        Self {
            dir: dir.to_path_buf(),
            python: Python::default(),
        }
    }
}

#[async_trait]
impl PluginTNG for Python2 {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        self.python.get_available_packages(&self.dir).await
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        self.python.get_downloaded_packages(&self.dir).await
    }

    async fn download_asset(&self, descriptor: &dyn Descriptor) -> IsopyLibResult<PathBuf> {
        self.python.download_asset(descriptor, &self.dir).await
    }
}

pub struct Python;

impl Python {
    async fn download_python(
        &self,
        descriptor: &PythonDescriptor,
        plugin_dir: &Path,
    ) -> IsopyLibResult<PathBuf> {
        let assets = Self::read_assets(plugin_dir)?;
        let asset = get_asset(&assets, descriptor)?;
        let repositories = Self::read_repositories(plugin_dir)?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;
        let assets_dir = plugin_dir.join(&*ASSETS_DIR);
        let asset_path = download_asset(repository, asset, &assets_dir).await?;
        Ok(asset_path)
    }

    fn read_assets(plugin_dir: &Path) -> Result<Vec<Asset>> {
        let index_path = plugin_dir.join(RELEASES_FILE_NAME);
        let package_recs = read_json_file::<Vec<PackageRec>>(&index_path)?;

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

    fn read_repositories(plugin_dir: &Path) -> Result<Vec<RepositoryInfo>> {
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

        let repositories_yaml_path = plugin_dir.join(REPOSITORIES_FILE_NAME);
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

    async fn update_index_if_necessary(&self, plugin_dir: &Path) -> Result<()> {
        let repositories = Self::read_repositories(plugin_dir)?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;

        let releases_path = Self::releases_path(&repository.name, plugin_dir);
        let current_last_modified = if releases_path.is_file() {
            Self::read_index_last_modified(&repository.name, plugin_dir)?
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
                Self::write_index_last_modified(&repository.name, last_modified, plugin_dir)?;
            }
        }

        Ok(())
    }

    fn get_available_packages_helper(plugin_dir: &Path) -> IsopyLibResult<Vec<Package>> {
        fn compare_by_version_and_tag(a: &Asset, b: &Asset) -> Ordering {
            match a.meta.version.cmp(&b.meta.version) {
                Ordering::Equal => a.tag.cmp(&b.tag),
                result => result,
            }
        }

        let mut assets = Self::read_assets(plugin_dir)?;
        assets.sort_by(|a, b| compare_by_version_and_tag(b, a));

        Ok(AssetFilter::default_for_platform()
            .filter(assets.iter())
            .into_iter()
            .map(|asset| Package {
                descriptor: Some(Arc::new(Box::new(PythonDescriptor {
                    version: asset.meta.version.clone(),
                    tag: Some(asset.tag.clone()),
                }))),
                asset_path: plugin_dir.join(&*ASSETS_DIR).join(asset.name.clone()),
            })
            .collect::<Vec<_>>())
    }

    fn releases_path(repository_name: &RepositoryName, plugin_dir: &Path) -> PathBuf {
        if repository_name.is_default() {
            plugin_dir.join(RELEASES_FILE_NAME)
        } else {
            label_file_name(
                &plugin_dir.join(RELEASES_FILE_NAME),
                repository_name.as_str(),
            )
            .expect("must be valid")
        }
    }

    fn read_index_last_modified(
        repository_name: &RepositoryName,
        plugin_dir: &Path,
    ) -> Result<Option<LastModified>> {
        let index_path = Self::index_path(repository_name, plugin_dir);
        Ok(if index_path.is_file() {
            Some(read_yaml_file::<IndexRec>(&index_path)?.last_modified)
        } else {
            None
        })
    }

    fn write_index_last_modified(
        repository_name: &RepositoryName,
        last_modified: &LastModified,
        plugin_dir: &Path,
    ) -> Result<()> {
        let index_yaml_path = Self::index_path(repository_name, plugin_dir);
        safe_write_file(
            &index_yaml_path,
            serde_yaml::to_string(&IndexRec {
                last_modified: last_modified.clone(),
            })?,
            true,
        )?;
        Ok(())
    }

    fn index_path(repository_name: &RepositoryName, plugin_dir: &Path) -> PathBuf {
        if repository_name.is_default() {
            plugin_dir.join(INDEX_FILE_NAME)
        } else {
            label_file_name(&plugin_dir.join(INDEX_FILE_NAME), repository_name.as_str())
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

    fn source_url(&self) -> &Url {
        &RELEASES_URL
    }

    async fn get_available_packages(&self, plugin_dir: &Path) -> IsopyLibResult<Vec<Package>> {
        self.update_index_if_necessary(plugin_dir).await?;
        Self::get_available_packages_helper(plugin_dir)
    }

    async fn get_downloaded_packages(&self, plugin_dir: &Path) -> IsopyLibResult<Vec<Package>> {
        let packages = self.get_available_packages(plugin_dir).await?;
        let package_map = packages
            .iter()
            .filter_map(|p| p.asset_path.file_name().map(OsString::from).map(|f| (f, p)))
            .collect::<HashMap<_, _>>();

        let assets_dir = plugin_dir.join(&*ASSETS_DIR);
        let mut packages = Vec::new();
        for result in read_dir(assets_dir).map_err(isopy_lib_other_error)? {
            let entry = result.map_err(isopy_lib_other_error)?;
            let asset_path = entry.path();
            let asset_file_name = entry.file_name();
            let descriptor = package_map
                .get(&asset_file_name)
                .and_then(|package| package.descriptor.as_ref())
                .cloned();
            if let Some(asset_file_name) = asset_file_name.to_str() {
                if asset_file_name.parse::<AssetMeta>().is_ok() {
                    packages.push(Package {
                        asset_path,
                        descriptor,
                    });
                }
            }
        }

        Ok(packages)
    }

    async fn download_asset(
        &self,
        descriptor: &dyn Descriptor,
        plugin_dir: &Path,
    ) -> IsopyLibResult<PathBuf> {
        let descriptor = descriptor
            .as_any()
            .downcast_ref::<PythonDescriptor>()
            .expect("must be PythonDescriptor");
        self.download_python(descriptor, plugin_dir).await
    }

    fn project_config_file_name(&self) -> &OsStr {
        &PROJECT_CONFIG_FILE_NAME
    }

    fn read_project_config_file(&self, path: &Path) -> IsopyLibResult<Box<dyn Descriptor>> {
        let project_config_rec =
            read_yaml_file::<ProjectConfigRec>(path).map_err(isopy_lib_other_error)?;

        Ok(Box::new(PythonDescriptor {
            version: project_config_rec.version,
            tag: project_config_rec.tag,
        }))
    }

    fn parse_descriptor(&self, s: &str) -> IsopyLibResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<PythonDescriptor>()
                .map_err(isopy_lib_other_error)?,
        ))
    }

    fn read_env_config(
        &self,
        data_dir: &Path,
        properties: &serde_json::Value,
    ) -> IsopyLibResult<EnvInfo> {
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
            .map_err(isopy_lib_other_error)?;

        Ok(EnvInfo {
            path_dirs: make_path_dirs(data_dir, &env_config_rec),
            envs: vec![],
        })
    }
}
