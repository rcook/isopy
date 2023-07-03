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

lazy_static! {
    pub static ref ADOPTIUM_SERVER_URL: Url =
        Url::parse("https://api.adoptium.net/").expect("lazy_static: URL must be valid");
    pub static ref RELEASES_URL: Url =
        Url::parse("https://api.github.com/repos/indygreg/python-build-standalone/releases")
            .expect("lazy_static: URL must be valid");
    pub static ref REPOSITORY_NAME_REGEX: Regex =
        Regex::new("^[A-Za-z0-9-_]+$").expect("lazy_static: regular expression must be valid");
}

pub const PYTHON_DESCRIPTOR_PREFIX: &str = "python";

pub const OPENJDK_DESCRIPTOR_PREFIX: &str = "openjdk";

pub const ADOPTIUM_INDEX_FILE_NAME: &str = "adoptium-index.yaml";

pub const DEFAULT_REPOSITORY_NAME: &str = "default";

pub const EXAMPLE_REPOSITORY_NAME: &str = "example";

pub type ExitCode = i32;

pub const OK: ExitCode = 0;

pub const ERROR: ExitCode = 1;

pub const CACHE_DIR_NAME: &str = ".isopy";

pub const PYTHON_VERSION_FILE_NAME: &str = ".python-version.yaml";

pub const OPENJDK_VERSION_FILE_NAME: &str = ".openjdk-version.yaml";

pub const REPOSITORIES_FILE_NAME: &str = "repositories.yaml";

pub const RELEASES_FILE_NAME: &str = "releases.json";

pub const INDEX_FILE_NAME: &str = "index.yaml";

pub const ENV_FILE_NAME: &str = "env.yaml";

pub const ISOPY_ENV_NAME: &str = "ISOPY_ENV";

pub const ISOPY_USER_AGENT: &str = "isopy";

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
