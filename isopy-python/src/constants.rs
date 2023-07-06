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
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use std::{ffi::OsString, path::PathBuf};

lazy_static! {
    pub static ref RELEASES_URL: Url =
        Url::parse("https://api.github.com/repos/indygreg/python-build-standalone/releases")
            .expect("lazy_static: URL must be valid");
    pub static ref REPOSITORY_NAME_REGEX: Regex =
        Regex::new("^[A-Za-z0-9-_]+$").expect("lazy_static: regular expression must be valid");
    pub static ref PROJECT_CONFIG_FILE_NAME: OsString = OsString::from(".python-version.yaml");
    pub static ref ENV_DIR: PathBuf = PathBuf::from("python");
    pub static ref ASSETS_DIR: PathBuf = PathBuf::from("assets");
}

pub const DEFAULT_REPOSITORY_NAME: &str = "default";

pub const EXAMPLE_REPOSITORY_NAME: &str = "example";

pub const REPOSITORIES_FILE_NAME: &str = "repositories.yaml";

pub const RELEASES_FILE_NAME: &str = "releases.json";

pub const INDEX_FILE_NAME: &str = "index.yaml";

pub const ISOPY_USER_AGENT: &str = "isopy";

pub const PLUGIN_NAME: &str = "Python";

#[cfg(test)]
mod tests {
    use super::{RELEASES_URL, REPOSITORY_NAME_REGEX};

    #[test]
    fn releases_url() {
        _ = RELEASES_URL.as_str().to_string();
    }

    #[test]
    fn repository_name_regex() {
        _ = REPOSITORY_NAME_REGEX.as_str().to_string();
    }
}
