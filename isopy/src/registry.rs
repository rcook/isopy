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
use crate::constants::PYTHON_DESCRIPTOR_PREFIX;
use crate::plugin_host::{PluginHost, PluginHostRef};
use crate::serialization::Package;
use anyhow::Result;
use isopy_lib::tng::EnvProps;
use isopy_lib::{EnvInfo, Platform, PluginFactory, Shell};
use isopy_python::PythonPluginFactory;
use lazy_static::lazy_static;
use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;

lazy_static! {
    static ref GLOBAL: Registry = Registry::new(vec![PluginHost::new(
        PYTHON_DESCRIPTOR_PREFIX,
        Box::<PythonPluginFactory>::default(),
    )]);
}

pub(crate) struct Registry {
    pub(crate) plugin_hosts: Vec<PluginHostRef>,
}

impl Registry {
    pub(crate) fn global() -> &'static Self {
        &GLOBAL
    }

    pub(crate) fn new(plugin_hosts: Vec<PluginHost>) -> Self {
        Self {
            plugin_hosts: plugin_hosts.into_iter().map(Arc::new).collect::<Vec<_>>(),
        }
    }

    pub(crate) fn make_env_info(
        &self,
        data_dir: &Path,
        package: &Package,
        base_dir: Option<&Path>,
    ) -> Result<Option<EnvInfo>> {
        let Some(plugin_host) = self
            .plugin_hosts
            .iter()
            .find(|p| p.prefix() == package.moniker)
        else {
            return Ok(None);
        };

        Ok(Some(plugin_host.make_env_info(
            data_dir,
            &EnvProps::new(&package.dir, &package.url),
            base_dir,
        )?))
    }

    pub(crate) fn make_script_command(
        &self,
        package: &Package,
        script_path: &Path,
        platform: Platform,
        shell: Shell,
    ) -> Result<Option<OsString>> {
        let Some(plugin_host) = self
            .plugin_hosts
            .iter()
            .find(|p| p.prefix() == package.moniker)
        else {
            return Ok(None);
        };

        Ok(plugin_host.make_script_command(script_path, platform, shell)?)
    }
}
