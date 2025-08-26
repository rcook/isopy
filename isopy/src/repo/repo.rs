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
use crate::error::HasOtherError;
use crate::formats::read_yaml_file;
use crate::fs::{FileReadError, read_text_file, safe_write_file};
use crate::repo::config::RepoConfig;
use crate::repo::dir_info::DirInfo;
use crate::repo::error::RepoError;
use crate::repo::link::{Link, LinkRecord};
use crate::repo::link_id::LinkId;
use crate::repo::manifest::{Manifest, ManifestRecord};
use crate::repo::meta_id::MetaId;
use crate::repo::result::RepoResult;
use crate::repo::shared_path::SharedPath;
use crate::repo::trash::Trash;
use chrono::Utc;
use fslock::LockFile;
use path_absolutize::Absolutize;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::{Path, PathBuf};

const MANIFEST_FILE_NAME: &str = "manifest.yaml";

#[derive(Debug)]
pub struct Repo {
    config: RepoConfig,
    _lock_file: LockFile,
}

impl Repo {
    pub fn new(config: RepoConfig) -> RepoResult<Option<Self>> {
        safe_write_file(&config.lock_path, vec![], true).map_err(RepoError::other)?;
        let mut lock_file = LockFile::open(&config.lock_path)
            .map_err(|_e| RepoError::could_not_open_lock_file(&config.lock_path))?;
        Ok(
            if lock_file
                .try_lock_with_pid()
                .map_err(|_e| RepoError::could_not_lock(&config.lock_path))?
            {
                Some(Self {
                    config,
                    _lock_file: lock_file,
                })
            } else {
                None
            },
        )
    }

    #[must_use]
    pub fn lock_path(&self) -> &Path {
        &self.config.lock_path
    }

    #[must_use]
    pub fn config_path(&self) -> &Path {
        &self.config.config_path
    }

    #[must_use]
    pub fn links_dir(&self) -> &Path {
        &self.config.links_dir
    }

    #[must_use]
    pub fn container_dir(&self) -> &Path {
        &self.config.container_dir
    }

    #[must_use]
    pub fn shared_dir(&self) -> &Path {
        &self.config.shared_dir
    }

    pub fn list_links(&self) -> RepoResult<Vec<Link>> {
        let mut links = Vec::new();

        if self.config.links_dir.is_dir() {
            for entry_opt in read_dir(&self.config.links_dir).map_err(RepoError::other)? {
                let entry = entry_opt.map_err(RepoError::other)?;
                if entry.path().is_file()
                    && let Some(link) = self.read_link_from_link_path(&entry.path())?
                {
                    links.push(link);
                }
            }
        }

        Ok(links)
    }

    pub fn list_manifests(&self) -> RepoResult<Vec<Manifest>> {
        let mut manifests = Vec::new();

        if self.config.container_dir.is_dir() {
            for entry_opt in read_dir(&self.config.container_dir).map_err(RepoError::other)? {
                let entry = entry_opt.map_err(RepoError::other)?;
                if entry.path().is_dir() {
                    manifests.push(self.read_manifest_from_datadir(&entry.path())?);
                }
            }
        }

        Ok(manifests)
    }

    pub fn init(&self, project_dir: &Path) -> RepoResult<Option<DirInfo>> {
        let link_id = Self::make_link_id(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if link_path.is_file() {
            return Ok(None);
        }

        let meta_id = MetaId::random();
        let data_dir = self.make_data_dir(&meta_id);
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);

        let manifest_record = ManifestRecord {
            created_at: Utc::now(),
            original_project_dir: project_dir.to_path_buf(),
            meta_id: meta_id.clone(),
        };
        let yaml_str = serde_yaml::to_string(&manifest_record).map_err(RepoError::other)?;
        safe_write_file(&manifest_path, yaml_str, false).map_err(RepoError::other)?;

        let link_record = LinkRecord {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id,
        };
        let yaml_str = serde_yaml::to_string(&link_record).map_err(RepoError::other)?;
        safe_write_file(&link_path, yaml_str, false).map_err(RepoError::other)?;

        Ok(Some(DirInfo {
            manifest: Manifest::new(data_dir, manifest_path, manifest_record),
            link: Link::new(link_path, link_record),
        }))
    }

    pub fn remove(&self, project_dir: &Path) -> RepoResult<bool> {
        Ok(if let Some(dir_info) = self.get(project_dir)? {
            remove_file(dir_info.link_path())
                .map_err(|_e| RepoError::could_not_delete_file(&self.config.config_path))?;
            Trash::compute(self)?.empty()?;
            true
        } else {
            false
        })
    }

    pub fn get(&self, project_dir: &Path) -> RepoResult<Option<DirInfo>> {
        let link_id = Self::make_link_id(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if !link_path.is_file() {
            return Ok(None);
        }

        let link_record = read_yaml_file::<LinkRecord>(&link_path).map_err(RepoError::other)?;
        if link_record.project_dir != *project_dir {
            return Err(RepoError::invalid_link_file(
                &link_path,
                &link_record.project_dir,
                project_dir,
            ));
        }

        let data_dir = self.make_data_dir(&link_record.meta_id);
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let manifest_record =
            read_yaml_file::<ManifestRecord>(&manifest_path).map_err(RepoError::other)?;

        Ok(Some(DirInfo {
            manifest: Manifest::new(data_dir, manifest_path, manifest_record),
            link: Link::new(link_path, link_record),
        }))
    }

    pub fn read_manifest(&self, meta_id: &MetaId) -> RepoResult<Manifest> {
        let manifest_path = self.make_data_dir(meta_id);
        self.read_manifest_from_datadir(&manifest_path)
    }

    #[allow(clippy::unused_self)]
    pub fn read_manifest_from_datadir(&self, data_dir: &Path) -> RepoResult<Manifest> {
        let manifest_path = data_dir.join(MANIFEST_FILE_NAME);
        let record = read_yaml_file(&manifest_path).map_err(RepoError::other)?;
        Ok(Manifest::new(data_dir.to_path_buf(), manifest_path, record))
    }

    #[allow(unused)]
    pub fn read_link(&self, project_dir: &Path) -> RepoResult<Option<Link>> {
        let link_id = Self::make_link_id(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        self.read_link_from_link_path(&link_path)
    }

    #[allow(clippy::unused_self)]
    pub fn read_link_from_link_path(&self, link_path: &Path) -> RepoResult<Option<Link>> {
        match read_yaml_file(link_path) {
            Ok(link_record) => Ok(Some(Link::new(link_path.to_path_buf(), link_record))),
            Err(e)
                if e.downcast_other_ref::<FileReadError>()
                    .is_some_and(FileReadError::is_not_found) =>
            {
                Ok(None)
            }
            Err(e) => Err(RepoError::other(e)),
        }
    }

    pub fn link(&self, meta_id: &MetaId, project_dir: &Path) -> RepoResult<Option<DirInfo>> {
        let manifest = self.read_manifest(meta_id)?;

        let link_id = Self::make_link_id(project_dir)?;
        let link_path = self.make_link_path(&link_id);
        if link_path.is_file() {
            return Ok(None);
        }

        let link_record = LinkRecord {
            created_at: Utc::now(),
            link_id,
            project_dir: project_dir.to_path_buf(),
            meta_id: meta_id.clone(),
        };
        let yaml_str = serde_yaml::to_string(&link_record).map_err(RepoError::other)?;
        safe_write_file(&link_path, yaml_str, false).map_err(RepoError::other)?;

        Ok(Some(DirInfo {
            manifest,
            link: Link::new(link_path, link_record),
        }))
    }

    #[allow(unused)]
    pub fn purge(&self) -> RepoResult<()> {
        if self.config.shared_dir.is_dir() {
            remove_dir_all(&self.config.shared_dir)
                .map_err(|_e| RepoError::could_not_delete_directory(&self.config.shared_dir))?;
        }
        if self.config.container_dir.is_dir() {
            remove_dir_all(&self.config.container_dir)
                .map_err(|_e| RepoError::could_not_delete_directory(&self.config.container_dir))?;
        }
        if self.config.links_dir.is_dir() {
            remove_dir_all(&self.config.links_dir)
                .map_err(|_e| RepoError::could_not_delete_directory(&self.config.links_dir))?;
        }
        if self.config.config_path.is_file() {
            remove_file(&self.config.config_path)
                .map_err(|_e| RepoError::could_not_delete_file(&self.config.config_path))?;
        }
        if self.config.lock_path.is_file() {
            remove_file(&self.config.lock_path)
                .map_err(|_e| RepoError::could_not_delete_file(&self.config.lock_path))?;
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn read_shared_file(&self, path: &SharedPath) -> RepoResult<Option<String>> {
        let p = self.resolve_shared_path(path)?;
        Ok(match read_text_file(&p) {
            Ok(s) => Some(s),
            Err(e) if e.is_not_found() => None,
            Err(e) => return Err(RepoError::other(e)),
        })
    }

    #[allow(unused)]
    pub fn write_shared_file(&self, path: &SharedPath, value: &str) -> RepoResult<()> {
        let p = self.resolve_shared_path(path)?;
        safe_write_file(&p, value, true).map_err(RepoError::other)?;
        Ok(())
    }

    fn make_link_id(project_dir: &Path) -> RepoResult<LinkId> {
        LinkId::try_from(project_dir)
    }

    fn make_link_path(&self, link_id: &LinkId) -> PathBuf {
        self.config.links_dir.join(format!("{link_id}.yaml"))
    }

    fn make_data_dir(&self, meta_id: &MetaId) -> PathBuf {
        self.config.container_dir.join(format!("{meta_id}"))
    }

    fn resolve_shared_path(&self, path: &SharedPath) -> RepoResult<PathBuf> {
        let p = Path::new(path.as_str())
            .absolutize_from(&self.config.shared_dir)
            .map_err(RepoError::other)?
            .into_owned();
        if !p.starts_with(&self.config.shared_dir) {
            return Err(RepoError::invalid_shared_path(path));
        }
        Ok(p)
    }
}
