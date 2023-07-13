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
use crate::descriptor_info::DescriptorInfo;
use crate::plugin_host::PluginHostRef;
use anyhow::Result;
use isopy_lib::Package;
use joatmon::{FileReadError, HasOtherError, YamlError};
use std::sync::Arc;

pub fn pretty_descriptor(plugin_host: &PluginHostRef, package: &Package) -> String {
    let descriptor_info = DescriptorInfo {
        plugin_host: Arc::clone(plugin_host),
        descriptor: Arc::clone(&package.descriptor),
    };
    descriptor_info.to_string()
}

pub fn existing<T>(result: Result<T>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(e) => {
            if let Some(e0) = e.downcast_ref::<YamlError>() {
                if let Some(e1) = e0.downcast_other_ref::<FileReadError>() {
                    if e1.is_not_found() {
                        return Ok(None);
                    }
                }
            }
            Err(e)
        }
    }
}
