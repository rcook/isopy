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
use crate::tng::java_package_manager::JavaPackageManager;
use anyhow::Result;
use async_trait::async_trait;
use isopy_lib2::tng::{Context, PackageManager, PackageManagerFactory, PackageManagerFactoryOps};

pub(crate) struct JavaPackageManagerFactory;

impl JavaPackageManagerFactory {
    pub(crate) async fn new() -> Result<PackageManagerFactory> {
        Ok(Box::new(Self))
    }
}

#[async_trait]
impl PackageManagerFactoryOps for JavaPackageManagerFactory {
    async fn make_package_manager(&self, _ctx: &dyn Context) -> Result<PackageManager> {
        let package_manager = JavaPackageManager::new();
        Ok(Box::new(package_manager))
    }
}
