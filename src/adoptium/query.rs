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
use crate::api::adoptium::{
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
        if let Some(value) = self.architecture.as_ref() {
            request_builder = request_builder.query(&[("architecture", value)]);
        }

        if let Some(value) = self.heap_size.as_ref() {
            request_builder = request_builder.query(&[("heap_size", value)]);
        }

        if let Some(value) = self.image_type.as_ref() {
            request_builder = request_builder.query(&[("image_type", value)]);
        }

        if let Some(value) = self.jvm_impl.as_ref() {
            request_builder = request_builder.query(&[("jvm_impl", value)]);
        }

        if let Some(value) = self.os.as_ref() {
            request_builder = request_builder.query(&[("os", value)]);
        }

        if let Some(value) = self.project.as_ref() {
            request_builder = request_builder.query(&[("project", value)]);
        }

        if let Some(value) = self.release_type.as_ref() {
            request_builder = request_builder.query(&[("release_type", value)]);
        }

        if let Some(value) = self.sort_method.as_ref() {
            request_builder = request_builder.query(&[("sort_method", value)]);
        }

        if let Some(value) = self.sort_order.as_ref() {
            request_builder = request_builder.query(&[("sort_order", value)]);
        }

        if let Some(value) = self.vendor.as_ref() {
            request_builder = request_builder.query(&[("vendor", value)]);
        }

        if let Some(value) = self.version.as_ref() {
            request_builder = request_builder.query(&[("version", value)]);
        }

        request_builder
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            #[cfg(target_arch = "aarch64")]
            architecture: Some(Architecture::AArch64),
            #[cfg(target_arch = "x86_64")]
            architecture: Some(Architecture::X64),
            heap_size: Some(HeapSize::Normal),
            image_type: Some(ImageType::Jre),
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
