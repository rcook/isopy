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
use joatmon::find_sentinel_file;
use std::path::{Path, PathBuf};

pub const ISOPY_DIR_NAME: &str = ".isopy";

pub const PROJECT_CONFIG_FILE_NAME: &str = ".python-version.yaml";

pub fn default_isopy_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir()?;
    let isopy_dir = home_dir.join(ISOPY_DIR_NAME);
    Some(isopy_dir)
}

pub fn make_project_config_path<P>(project_dir: P) -> PathBuf
where
    P: Into<PathBuf>,
{
    project_dir.into().join(PROJECT_CONFIG_FILE_NAME)
}

pub fn find_project_config_path<P>(start_dir: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    find_sentinel_file(PROJECT_CONFIG_FILE_NAME, &start_dir, None)
}
