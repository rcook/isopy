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
use crate::tng::python_manager::PythonManager;
use anyhow::Result;
use isopy_lib::tng::{Context, Manager, Plugin, PluginOps, Version, VersionTriple};
use std::sync::Arc;

pub(crate) struct PythonPlugin {
    ctx: Context,
}

impl PythonPlugin {
    pub(crate) fn new(ctx: Context) -> Plugin {
        Box::new(Self { ctx })
    }
}

impl PluginOps for PythonPlugin {
    fn parse_version(&self, s: &str) -> Result<Version> {
        Ok(Box::new(s.parse::<VersionTriple>()?))
    }

    fn new_manager(&self) -> Manager {
        Box::new(PythonManager::new(Arc::clone(&self.ctx)))
    }
}
