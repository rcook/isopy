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
use async_trait::async_trait;
use isopy_lib::{Descriptor, IsopyLibResult, Package, Plugin};

pub struct GoPlugin;

impl GoPlugin {
    pub const fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Plugin for GoPlugin {
    async fn get_available_packages(&self) -> IsopyLibResult<Vec<Package>> {
        Ok(vec![])
    }

    async fn get_downloaded_packages(&self) -> IsopyLibResult<Vec<Package>> {
        Ok(vec![])
    }

    async fn download_package(&self, _descriptor: &dyn Descriptor) -> IsopyLibResult<Package> {
        todo!();
    }
}
