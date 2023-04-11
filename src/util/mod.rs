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
mod constants;
mod download_file;
mod fs;
mod hash;
mod helpers;
mod indicator;
mod probe;
mod sha256sums;
mod str;
mod time;
mod types;
mod unpack_file;
mod url;

pub use self::constants::{GENERAL_ERROR, OK, RELEASES_URL, USAGE};
pub use self::download_file::download_stream;
pub use self::fs::{open_file, read_json_file, read_yaml_file, safe_create_file, safe_write_file};
pub use self::hash::HexDigest;
pub use self::helpers::{download_asset, get_asset};
pub use self::indicator::Indicator;
pub use self::probe::{
    default_isopy_dir, find_project_config_path, make_project_config_path, PROJECT_CONFIG_FILE_NAME,
};
pub use self::sha256sums::validate_sha256_checksum;
pub use self::str::{osstr_to_str, osstring_to_string, path_to_str};
pub use self::time::{to_last_modified, to_system_time};
pub use self::types::ContentLength;
pub use self::unpack_file::unpack_file;
pub use self::url::{dir_url, file_url};
