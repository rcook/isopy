use crate::serialization::PackageDirRec;

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
use super::descriptor_id::DescriptorId;
use super::descriptor_info::DescriptorInfo;
use super::product_info::ProductInfo;
use anyhow::{anyhow, Result};
use isopy_lib::{EnvInfo, ParseDescriptorError, ParseDescriptorResult};
use std::path::Path;
use std::rc::Rc;

pub struct ProductRegistry {
    pub product_infos: Vec<Rc<ProductInfo>>,
}

impl ProductRegistry {
    pub fn new(product_infos: Vec<ProductInfo>) -> Self {
        Self {
            product_infos: product_infos.into_iter().map(Rc::new).collect::<Vec<_>>(),
        }
    }

    pub fn to_descriptor_info(
        &self,
        descriptor_id: &DescriptorId,
    ) -> ParseDescriptorResult<DescriptorInfo> {
        let s = descriptor_id.as_str();

        let Some((product_info, tail)) = self.find_product_info(s) else {
            return Err(ParseDescriptorError::Other(anyhow!("unsupported descriptor format {s}")));
        };

        let descriptor = product_info.product.parse_descriptor(tail)?;
        Ok(DescriptorInfo {
            product_info: Rc::clone(product_info),
            descriptor,
        })
    }

    pub fn blah(
        &self,
        data_dir: &Path,
        package_dir_rec: &PackageDirRec,
    ) -> Result<Option<EnvInfo>> {
        let Some(product_info) = self
            .product_infos
            .iter()
            .find(|p| p.prefix == package_dir_rec.id) else {
            return Ok(None);
        };

        Ok(Some(
            product_info
                .product
                .read_env_config(data_dir, &package_dir_rec.properties)?,
        ))
    }

    fn find_product_info<'a>(&self, s: &'a str) -> Option<(&Rc<ProductInfo>, &'a str)> {
        if let Some((prefix, tail)) = s.split_once(':') {
            if let Some(product_info) = self.product_infos.iter().find(|p| p.prefix == prefix) {
                return Some((product_info, tail));
            }
        }

        self.product_infos.first().map(|p| (p, s))
    }
}

#[cfg(test)]
mod tests {
    use super::{ProductInfo, ProductRegistry};
    use crate::registry::DescriptorId;
    use anyhow::Result;
    use isopy_openjdk::OpenJdk;
    use isopy_python::Python;
    use rstest::rstest;

    #[rstest]
    #[case("python:1.2.3:11223344", "1.2.3:11223344")]
    #[case("python:1.2.3:11223344", "python:1.2.3:11223344")]
    #[case("openjdk:19.0.1+10", "openjdk:19.0.1+10")]
    fn to_descriptor_info(#[case] expected_str: &str, #[case] input: &str) -> Result<()> {
        let registry = ProductRegistry::new(vec![
            ProductInfo {
                prefix: String::from("python"),
                product: Box::<Python>::default(),
            },
            ProductInfo {
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
