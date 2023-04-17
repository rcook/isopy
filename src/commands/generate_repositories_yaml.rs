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
use crate::object_model::RepositoryName;
use crate::serialization::{RepositoriesRecord, RepositoryRecord};
use crate::util::{dir_url, RELEASES_URL};
use anyhow::Result;
use joatmon::safe_write_file;
use std::path::{Path, PathBuf};

pub fn do_generate_repositories_yaml<P>(app: &App, local_repository_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    safe_write_file(
        app.dir.join("repositories.yaml"),
        serde_yaml::to_string(&RepositoriesRecord {
            repositories: vec![
                RepositoryRecord::GitHub {
                    name: RepositoryName::parse("default").expect("must parse"),
                    url: dir_url(RELEASES_URL)?,
                    enabled: true,
                },
                RepositoryRecord::Local {
                    name: RepositoryName::parse("local").expect("must parse"),
                    dir: PathBuf::from(local_repository_dir.as_ref()),
                    enabled: true,
                },
            ],
        })?,
        false,
    )?;
    Ok(())
}
