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
mod accept;
mod checksum;
mod download_options;
mod env_info;
mod env_props;
mod file_name_parts;
mod macros;
mod package;
mod package_filter;
mod package_kind;
mod package_manager;
mod package_manager_context;
mod package_summary;
mod plugin;
mod sanitize;
mod shell;
mod tags;
mod version;
mod version_triple;

pub use accept::Accept;
pub use checksum::Checksum;
pub use download_options::DownloadOptions;
pub use env_info::EnvInfo;
pub use env_props::EnvProps;
pub use file_name_parts::FileNameParts;
pub use macros::TryToString;
pub use package::{Package, PackageOps};
pub use package_filter::PackageFilter;
pub use package_kind::PackageKind;
pub use package_manager::{OptionalTags, PackageManager, PackageManagerOps};
pub use package_manager_context::{PackageManagerContext, PackageManagerContextOps};
pub use package_summary::PackageSummary;
pub use plugin::{Plugin, PluginOps};
pub use sanitize::{sanitize, sanitize_with_options, SanitizeOptions};
pub use shell::{env_var_substitution, join_paths, render_absolute_path, Platform, Shell};
pub use tags::Tags;
pub use version::Version;
pub use version_triple::VersionTriple;
