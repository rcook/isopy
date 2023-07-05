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
pub mod constants;
pub mod python_standalone_builds;
pub mod serialization;

mod asset;
mod asset_filter;
mod asset_helper;
mod asset_meta;
mod checksum;
mod foo;
mod github;
mod local;
mod python;
mod python_descriptor;
mod python_version;
mod repository_info;
mod repository_name;
mod tag;
mod traits;

pub use self::asset::Asset;
pub use self::asset_filter::AssetFilter;
pub use self::asset_helper::{download_asset, get_asset};
pub use self::asset_meta::AssetMeta;
pub use self::checksum::validate_sha256_checksum;
pub use self::foo::download_python;
pub use self::github::GitHubRepository;
pub use self::local::LocalRepository;
pub use self::python::Python;
pub use self::python_descriptor::PythonDescriptor;
pub use self::python_version::{PythonVersion, PythonVersionParseError, PythonVersionParseResult};
pub use self::repository_info::RepositoryInfo;
pub use self::repository_name::RepositoryName;
pub use self::tag::{option_tag, Tag};
pub use self::traits::Repository;
