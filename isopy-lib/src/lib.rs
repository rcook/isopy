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
#![warn(clippy::all)]
//#![warn(clippy::cargo)]
//#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
//#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::future_not_send)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::option_if_let_else)]
mod checksum;
mod descriptor;
mod download;
mod env_info;
mod error;
mod last_modified;
mod link_header;
mod package_info;
mod product;
mod reqwest_response;
mod response;
mod result;
mod url;

pub use self::checksum::verify_sha256_file_checksum;
pub use self::descriptor::{Descriptor, ProjectConfigInfo};
pub use self::download::download_stream;
pub use self::env_info::EnvInfo;
pub use self::error::IsopyLibError;
pub use self::last_modified::LastModified;
pub use self::link_header::{LinkHeader, LinkHeaderParseError, LinkHeaderParseErrorResult};
pub use self::package_info::PackageInfo;
pub use self::product::Product;
pub use self::reqwest_response::ReqwestResponse;
pub use self::response::{ContentLength, Response, Stream};
pub use self::result::IsopyLibResult;
pub use self::url::{dir_url, file_url};
