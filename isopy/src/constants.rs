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
pub(crate) const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const PACKAGE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const PACKAGE_HOME_PAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub(crate) const PACKAGE_BUILD_VERSION: Option<&str> =
    option_env!("RUST_TOOL_ACTION_BUILD_VERSION");

pub(crate) const ENV_CONFIG_FILE_NAME: &str = "env.yaml";
pub(crate) const PROJECT_CONFIG_FILE_NAME: &str = ".isopy.yaml";
pub(crate) const CACHE_DIR_NAME: &str = "cache";
pub(crate) const DOWNLOAD_CACHE_FILE_NAME: &str = "downloads.yaml";
pub(crate) const CONFIG_DIR_NAME: &str = "isopy";
pub(crate) const ISOPY_USER_AGENT: &str = "isopy";
pub(crate) const DEFAULT_MONIKER_CONFIG_NAME: &str = "default_moniker";
pub(crate) const CONFIG_NAMES: [&str; 1] = [DEFAULT_MONIKER_CONFIG_NAME];

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) const EXECUTABLE_MASK: u32 = 0o100;
