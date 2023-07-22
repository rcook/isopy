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
use super::super::api::{
    AdoptiumJvmImpl, AdoptiumVendor, Architecture, HeapSize, ImageType, OperatingSystem, Project,
    ReleaseType, SortMethod, SortOrder,
};
use reqwest::RequestBuilder;

pub struct Query {
    pub architecture: Option<Architecture>,
    pub heap_size: Option<HeapSize>,
    pub image_type: Option<ImageType>,
    pub jvm_impl: Option<AdoptiumJvmImpl>,
    pub os: Option<OperatingSystem>,
    pub project: Option<Project>,
    pub release_type: Option<ReleaseType>,
    pub sort_method: Option<SortMethod>,
    pub sort_order: Option<SortOrder>,
    pub vendor: Option<AdoptiumVendor>,
    pub version: Option<String>,
}

impl Query {
    pub fn apply(&self, mut request_builder: RequestBuilder) -> RequestBuilder {
        macro_rules! query_field {
            ($field: ident) => {
                if let Some(value) = self.$field.as_ref() {
                    request_builder = request_builder.query(&[(stringify!($field), value)]);
                }
            };
        }

        query_field!(architecture);
        query_field!(heap_size);
        query_field!(image_type);
        query_field!(jvm_impl);
        query_field!(os);
        query_field!(project);
        query_field!(release_type);
        query_field!(sort_method);
        query_field!(sort_order);
        query_field!(vendor);
        query_field!(version);
        request_builder
    }
}

impl Query {
    pub const fn new(image_type: ImageType) -> Self {
        Self {
            #[cfg(target_arch = "aarch64")]
            architecture: Some(Architecture::AArch64),
            #[cfg(target_arch = "x86_64")]
            architecture: Some(Architecture::X64),
            heap_size: Some(HeapSize::Normal),
            image_type: Some(image_type),
            jvm_impl: Some(AdoptiumJvmImpl::Hotspot),
            #[cfg(target_os = "linux")]
            os: Some(OperatingSystem::Linux),
            #[cfg(target_os = "macos")]
            os: Some(OperatingSystem::Mac),
            #[cfg(target_os = "windows")]
            os: Some(OperatingSystem::Windows),
            project: Some(Project::Jdk),
            release_type: Some(ReleaseType::Ga),
            sort_method: Some(SortMethod::Default),
            sort_order: Some(SortOrder::Desc),
            vendor: Some(AdoptiumVendor::Eclipse),
            version: None,
        }
    }
}
