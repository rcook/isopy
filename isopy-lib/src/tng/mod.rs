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
mod accept;
mod checksum;
mod download_options;
mod file_name_parts;
mod host;
mod manager;
mod plugin;
mod sanitize;
mod version;
mod version_triple;

pub use accept::Accept;
pub use checksum::Checksum;
pub use download_options::DownloadOptions;
pub use file_name_parts::FileNameParts;
pub use host::{Host, HostOps};
pub use manager::{Manager, ManagerOps};
pub use plugin::{Plugin, PluginOps};
pub use sanitize::{sanitize, sanitize_with_options, SanitizeOptions};
pub use version::Version;
pub use version_triple::VersionTriple;
