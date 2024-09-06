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
use crate::tng::python_package_manager::PythonPackageManager;
use anyhow::Result;
use isopy_lib::tng::{
    PackageManager, PackageManagerContext, Plugin, PluginOps, Version, VersionTriple,
};

pub(crate) struct PythonPlugin;

impl PythonPlugin {
    pub(crate) fn new() -> Plugin {
        Plugin::new(Box::new(Self))
    }
}

impl PluginOps for PythonPlugin {
    fn parse_version(&self, s: &str) -> Result<Version> {
        Ok(Version::new(Box::new(s.parse::<VersionTriple>()?)))
    }

    fn new_package_manager(&self, ctx: PackageManagerContext) -> PackageManager {
        PackageManager::new(Box::new(PythonPackageManager::new(ctx)))
    }
}
