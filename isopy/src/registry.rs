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
use crate::constants::{OPENJDK_DESCRIPTOR_PREFIX, PYTHON_DESCRIPTOR_PREFIX};
use crate::descriptor_info::DescriptorInfo;
use crate::plugin_host::PluginHost;
use crate::serialization::PackageRec;
use anyhow::{bail, Result};
use isopy_lib::EnvInfo;
use isopy_openjdk::OpenJdkPluginFactory;
use isopy_python::PythonPluginFactory;
use lazy_static::lazy_static;
use std::path::Path;
use std::sync::Arc;

lazy_static! {
    static ref GLOBAL: Registry = Registry::new(vec![
        PluginHost::new(
            PYTHON_DESCRIPTOR_PREFIX,
            Box::<PythonPluginFactory>::default(),
        ),
        PluginHost::new(
            OPENJDK_DESCRIPTOR_PREFIX,
            Box::<OpenJdkPluginFactory>::default(),
        )
    ]);
}

pub struct Registry {
    pub plugin_hosts: Vec<Arc<PluginHost>>,
}

impl Registry {
    pub fn global() -> &'static Self {
        &GLOBAL
    }

    pub fn new(plugin_hosts: Vec<PluginHost>) -> Self {
        Self {
            plugin_hosts: plugin_hosts.into_iter().map(Arc::new).collect::<Vec<_>>(),
        }
    }

    pub fn parse_descriptor(&self, s: &str) -> Result<DescriptorInfo> {
        let Some((plugin_host, tail)) = self.find_plugin_host(s) else {
            bail!("unsupported descriptor format {s}");
        };

        let descriptor = Arc::new(plugin_host.parse_descriptor(tail)?);

        Ok(DescriptorInfo {
            plugin_host: Arc::clone(plugin_host),
            descriptor,
        })
    }

    pub fn get_env_info(
        &self,
        data_dir: &Path,
        package_rec: &PackageRec,
    ) -> Result<Option<EnvInfo>> {
        let Some(plugin_host) = self
            .plugin_hosts
            .iter()
            .find(|p| p.prefix() == package_rec.id) else {
            return Ok(None);
        };

        Ok(Some(
            plugin_host.read_env_config(data_dir, &package_rec.properties)?,
        ))
    }

    fn find_plugin_host<'a>(&self, s: &'a str) -> Option<(&Arc<PluginHost>, &'a str)> {
        if let Some((prefix, tail)) = s.split_once(':') {
            if let Some(plugin_host) = self.plugin_hosts.iter().find(|p| p.prefix() == prefix) {
                return Some((plugin_host, tail));
            }
        }

        self.plugin_hosts.first().map(|p| (p, s))
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{OPENJDK_DESCRIPTOR_PREFIX, PYTHON_DESCRIPTOR_PREFIX};
    use crate::plugin_host::PluginHost;
    use crate::registry::Registry;
    use anyhow::Result;
    use isopy_openjdk::OpenJdkPluginFactory;
    use isopy_python::PythonPluginFactory;
    use rstest::rstest;

    #[rstest]
    #[case("python:1.2.3:11223344", "1.2.3:11223344")]
    #[case("python:1.2.3:11223344", "python:1.2.3:11223344")]
    #[case("openjdk:19.0.1+10", "openjdk:19.0.1+10")]
    fn to_descriptor_info(#[case] expected_str: &str, #[case] input: &str) -> Result<()> {
        let registry = Registry::new(vec![
            PluginHost::new(
                PYTHON_DESCRIPTOR_PREFIX,
                Box::<PythonPluginFactory>::default(),
            ),
            PluginHost::new(
                OPENJDK_DESCRIPTOR_PREFIX,
                Box::<OpenJdkPluginFactory>::default(),
            ),
        ]);

        let descriptor_info = registry.parse_descriptor(input)?;
        assert_eq!(expected_str, descriptor_info.to_string());
        Ok(())
    }
}
