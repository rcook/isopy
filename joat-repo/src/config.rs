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
use crate::error::RepoError;
use crate::repo::Repo;
use crate::result::RepoResult;
use joatmon::{read_yaml_file, safe_write_file};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoConfig {
    pub lock_path: PathBuf,
    pub config_path: PathBuf,
    pub links_dir: PathBuf,
    pub container_dir: PathBuf,
    pub shared_dir: PathBuf,
}

impl RepoConfig {
    #[must_use]
    pub fn default(base_dir: &Path, prefix: Option<&str>) -> Self {
        let full_prefix = prefix.map(|s| format!("{s}-")).unwrap_or_default();
        Self {
            lock_path: base_dir.join(format!(".{full_prefix}lock")),
            config_path: base_dir.join(format!("{full_prefix}config.yaml")),
            links_dir: base_dir.join(format!("{full_prefix}links")),
            container_dir: base_dir.join(format!("{full_prefix}data")),
            shared_dir: base_dir.join(format!("{full_prefix}shared")),
        }
    }

    pub fn repo(self) -> RepoResult<Option<Repo>> {
        Repo::new(if self.config_path.is_file() {
            read_yaml_file::<Self>(&self.config_path).map_err(RepoError::other)?
        } else {
            let yaml_str = serde_yaml::to_string(&self).map_err(RepoError::other)?;
            safe_write_file(&self.config_path, yaml_str, false).map_err(RepoError::other)?;
            self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tempdir::TempDir;

    #[test]
    fn default() -> Result<()> {
        let base_dir = TempDir::new("joat-repo-test")?;
        let c = RepoConfig::default(base_dir.path(), None);
        assert_eq!(base_dir.path().join(".lock"), c.lock_path);
        assert_eq!(base_dir.path().join("config.yaml"), c.config_path);
        assert_eq!(base_dir.path().join("links"), c.links_dir);
        assert_eq!(base_dir.path().join("data"), c.container_dir);
        assert_eq!(base_dir.path().join("shared"), c.shared_dir);
        Ok(())
    }

    #[test]
    fn prefix() -> Result<()> {
        let base_dir = TempDir::new("joat-repo-test")?;
        let c = RepoConfig::default(base_dir.path(), Some("foo"));
        assert_eq!(base_dir.path().join(".foo-lock"), c.lock_path);
        assert_eq!(base_dir.path().join("foo-config.yaml"), c.config_path);
        assert_eq!(base_dir.path().join("foo-links"), c.links_dir);
        assert_eq!(base_dir.path().join("foo-data"), c.container_dir);
        assert_eq!(base_dir.path().join("foo-shared"), c.shared_dir);
        Ok(())
    }
}
