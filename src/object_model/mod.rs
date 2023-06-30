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
mod arch;
mod archive_type;
mod asset;
mod asset_filter;
mod asset_meta;
mod family;
mod flavour;
mod last_modified;
mod maven_version;
mod openjdk_version;
mod os;
mod platform;
mod repository_name;
mod subflavour;
mod tag;
mod variant;
mod version;

pub use self::arch::Arch;
pub use self::archive_type::ArchiveType;
pub use self::asset::Asset;
pub use self::asset_filter::AssetFilter;
pub use self::asset_meta::AssetMeta;
pub use self::family::Family;
pub use self::flavour::Flavour;
pub use self::last_modified::LastModified;
pub use self::maven_version::{MavenVersion, MavenVersionLimit, MavenVersionValue};
pub use self::openjdk_version::{
    OpenJdkVersion, OpenJdkVersionKind, OpenJdkVersionParseError, OpenJdkVersionParseResult,
};
pub use self::os::OS;
pub use self::platform::Platform;
pub use self::repository_name::RepositoryName;
pub use self::subflavour::Subflavour;
pub use self::tag::option_tag;
pub use self::tag::Tag;
pub use self::variant::Variant;
pub use self::version::Version;
