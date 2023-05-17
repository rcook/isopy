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
pub const RELEASES_URL: &str =
    "https://api.github.com/repos/indygreg/python-build-standalone/releases";

pub type ExitCode = i32;

pub const OK: ExitCode = 0;

pub const ERROR: ExitCode = 1;

pub const CACHE_DIR_NAME: &str = ".isopy";

pub const PROJECT_CONFIG_FILE_NAME: &str = ".python-version.yaml";

pub const REPOSITORIES_FILE_NAME: &str = "repositories.yaml";

pub const INDEX_FILE_NAME: &str = "releases.json";

pub const INDEX_YAML_FILE_NAME: &str = "index.yaml";
