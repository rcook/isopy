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
use crate::descriptor_id::DescriptorId;
use crate::descriptor_info::DescriptorInfo;
use crate::plugin::Plugin;
use crate::serialization::PackageDirRec;
use anyhow::{anyhow, Result};
use isopy_lib::{EnvInfo, ParseDescriptorError, ParseDescriptorResult};
use std::path::Path;
use std::rc::Rc;

pub struct Registry {
    pub plugins: Vec<Rc<Plugin>>,
}

impl Registry {
    pub fn new(plugins: Vec<Plugin>) -> Self {
        Self {
            plugins: plugins.into_iter().map(Rc::new).collect::<Vec<_>>(),
        }
    }

    pub fn to_descriptor_info(
        &self,
        descriptor_id: &DescriptorId,
    ) -> ParseDescriptorResult<DescriptorInfo> {
        let s = descriptor_id.as_str();

        let Some((plugin, tail)) = self.find_plugin(s) else {
            return Err(ParseDescriptorError::Other(anyhow!("unsupported descriptor format {s}")));
        };

        let descriptor = plugin.product.parse_descriptor(tail)?;
        Ok(DescriptorInfo {
            plugin: Rc::clone(plugin),
            descriptor,
        })
    }

    pub fn blah(
        &self,
        data_dir: &Path,
        package_dir_rec: &PackageDirRec,
    ) -> Result<Option<EnvInfo>> {
        let Some(plugin) = self
            .plugins
            .iter()
            .find(|p| p.prefix == package_dir_rec.id) else {
            return Ok(None);
        };

        Ok(Some(
            plugin
                .product
                .read_env_config(data_dir, &package_dir_rec.properties)?,
        ))
    }

    fn find_plugin<'a>(&self, s: &'a str) -> Option<(&Rc<Plugin>, &'a str)> {
        if let Some((prefix, tail)) = s.split_once(':') {
            if let Some(plugin) = self.plugins.iter().find(|p| p.prefix == prefix) {
                return Some((plugin, tail));
            }
        }

        self.plugins.first().map(|p| (p, s))
    }
}

#[cfg(test)]
mod tests {
    use crate::descriptor_id::DescriptorId;
    use crate::plugin::Plugin;
    use crate::registry::Registry;
    use anyhow::Result;
    use isopy_openjdk::OpenJdk;
    use isopy_python::Python;
    use rstest::rstest;

    #[rstest]
    #[case("python:1.2.3:11223344", "1.2.3:11223344")]
    #[case("python:1.2.3:11223344", "python:1.2.3:11223344")]
    #[case("openjdk:19.0.1+10", "openjdk:19.0.1+10")]
    fn to_descriptor_info(#[case] expected_str: &str, #[case] input: &str) -> Result<()> {
        let registry = Registry::new(vec![
            Plugin {
                prefix: String::from("python"),
                product: Box::<Python>::default(),
            },
            Plugin {
                prefix: String::from("openjdk"),
                product: Box::<OpenJdk>::default(),
            },
        ]);

        let descriptor_id = input.parse::<DescriptorId>()?;
        let descriptor_info = registry.to_descriptor_info(&descriptor_id)?;
        assert_eq!(expected_str, descriptor_info.to_string());
        Ok(())
    }
}
