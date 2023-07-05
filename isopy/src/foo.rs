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
use crate::serialization::{PackageRec, RepositoriesRec, RepositoryRec};
use anyhow::{anyhow, Result};
use isopy_lib::{dir_url, download_stream, PackageInfo};
use isopy_python::constants::{RELEASES_FILE_NAME, RELEASES_URL, REPOSITORIES_FILE_NAME};
use isopy_python::AssetFilter;
use isopy_python::{
    download_asset, get_asset, Asset, AssetMeta, GitHubRepository, LocalRepository,
    PythonDescriptor, Repository, RepositoryInfo, RepositoryName,
};
use joatmon::{read_json_file, read_yaml_file, safe_write_file};
use std::cmp::Ordering;
use std::path::{Path, PathBuf};

pub struct Foo;

impl Foo {
    pub const fn new() -> Self {
        Self
    }

    pub fn read_assets(shared_dir: &Path) -> Result<Vec<Asset>> {
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

    pub fn read_repositories(shared_dir: &Path) -> Result<Vec<RepositoryInfo>> {
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

    pub async fn download_python(
        &self,
        descriptor: &PythonDescriptor,
        shared_dir: &Path,
    ) -> Result<PathBuf> {
        let assets = Self::read_assets(shared_dir)?;
        let asset = get_asset(&assets, descriptor)?;
        let repositories = Self::read_repositories(shared_dir)?;
        let repository = repositories
            .first()
            .ok_or_else(|| anyhow!("No asset repositories are configured"))?;
        let asset_path = download_asset(repository, asset, shared_dir).await?;
        Ok(asset_path)
    }
}

#[allow(unused)]
async fn show_python_index(app: &App) -> Result<Vec<PackageInfo>> {
    update_index_if_necessary(app).await?;
    show_available_downloads(app)
}

async fn update_index_if_necessary(app: &App) -> Result<()> {
    let repositories = Foo::read_repositories(app.repo.shared_dir())?;
    let repository = repositories
        .first()
        .ok_or_else(|| anyhow!("No asset repositories are configured"))?;

    let releases_path = app.releases_path(&repository.name);
    let current_last_modified = if releases_path.is_file() {
        app.read_index_last_modified(&repository.name)?
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
            app.write_index_last_modified(&repository.name, last_modified)?;
        }
    }

    Ok(())
}

fn show_available_downloads(app: &App) -> Result<Vec<PackageInfo>> {
    fn compare_by_version_and_tag(a: &Asset, b: &Asset) -> Ordering {
        match a.meta.version.cmp(&b.meta.version) {
            Ordering::Equal => a.tag.cmp(&b.tag),
            result => result,
        }
    }

    let mut assets = Foo::read_assets(app.repo.shared_dir())?;
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
