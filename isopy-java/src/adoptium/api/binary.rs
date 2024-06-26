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
use super::adoptium_jvm_impl::AdoptiumJvmImpl;
use super::architecture::Architecture;
use super::heap_size::HeapSize;
use super::image_type::ImageType;
use super::operating_system::OperatingSystem;
use super::package::Package;
use super::project::Project;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct Binary {
    #[serde(rename = "os")]
    pub os: OperatingSystem,

    #[serde(rename = "architecture")]
    pub architecture: Architecture,

    #[serde(rename = "image_type")]
    pub image_type: ImageType,

    #[serde(rename = "jvm_impl")]
    pub jvm_impl: AdoptiumJvmImpl,

    #[serde(rename = "package")]
    pub package: Option<Package>,

    #[serde(rename = "heap_size")]
    pub heap_size: HeapSize,

    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    #[serde(rename = "project")]
    pub project: Project,
}
