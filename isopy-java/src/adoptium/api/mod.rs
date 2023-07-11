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
mod adoptium_jvm_impl;
mod adoptium_vendor;
mod architecture;
mod binary;
mod heap_size;
mod image_type;
mod list;
mod operating_system;
mod package;
mod project;
mod release;
mod release_type;
mod singleton;
mod sort_method;
mod sort_order;
mod version_data;
mod versions;

pub use self::adoptium_jvm_impl::AdoptiumJvmImpl;
pub use self::adoptium_vendor::AdoptiumVendor;
pub use self::architecture::Architecture;
pub use self::binary::Binary;
pub use self::heap_size::HeapSize;
pub use self::image_type::ImageType;
pub use self::list::List;
pub use self::operating_system::OperatingSystem;
pub use self::package::Package;
pub use self::project::Project;
pub use self::release::Release;
pub use self::release_type::ReleaseType;
pub use self::singleton::Singleton;
pub use self::sort_method::SortMethod;
pub use self::sort_order::SortOrder;
pub use self::version_data::VersionData;
pub use self::versions::Versions;
