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
use crate::constants::{ENV_FILE_NAME, OPENJDK_DESCRIPTOR_PREFIX, PYTHON_DESCRIPTOR_PREFIX};
use crate::foo::Foo;
use crate::registry::{ProductInfo, ProductRegistry};
use crate::serialization::IndexRec;
use crate::serialization::{EnvRec, PackageDirRec};
use crate::unpack::unpack_file;
use anyhow::{bail, Result};
use isopy_lib::{Descriptor, LastModified};
use isopy_openjdk::OpenJdk;
use isopy_python::constants::{INDEX_FILE_NAME, RELEASES_FILE_NAME};
use isopy_python::{Python, PythonDescriptor, RepositoryName};
use joat_repo::{DirInfo, Link, LinkId, Repo, RepoResult};
use joatmon::{label_file_name, read_yaml_file, safe_write_file};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct App {
    pub cwd: PathBuf,
    pub repo: Repo,
    pub registry: ProductRegistry,
    pub foo: Foo,
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
            foo: Foo::new(),
        }
    }

    pub async fn download_asset(
        &self,
        product_info: &ProductInfo,
        descriptor: &dyn Descriptor,
        shared_dir: &Path,
    ) -> Result<PathBuf> {
        if let Some(d) = descriptor.as_any().downcast_ref::<PythonDescriptor>() {
            // TBD: Push into Product::download_asset
            Ok(self.foo.download_python(d, shared_dir).await?)
        } else {
            Ok(product_info
                .product
                .download_asset(descriptor, shared_dir)
                .await?)
        }
    }

    #[allow(unused)]
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

    #[allow(unused)]
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

    #[allow(unused)]
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

    pub async fn init_project(
        &self,
        product_info: &ProductInfo,
        descriptor: &dyn Descriptor,
    ) -> Result<()> {
        let asset_path = self
            .download_asset(product_info, descriptor, self.repo.shared_dir())
            .await?;

        let Some(dir_info) = self.repo.init(&self.cwd)? else {
            bail!(
                "Could not initialize metadirectory for directory {}",
                self.cwd.display()
            )
        };

        unpack_file(descriptor, &asset_path, dir_info.data_dir())?;

        safe_write_file(
            &dir_info.data_dir().join(ENV_FILE_NAME),
            serde_yaml::to_string(&EnvRec {
                config_path: self.cwd.clone(),
                package_dirs: vec![PackageDirRec {
                    id: product_info.prefix.clone(),
                    properties: descriptor.get_env_config_value()?,
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

    #[allow(unused)]
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
