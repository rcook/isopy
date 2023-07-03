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
use anyhow::anyhow;
use isopy_lib::{Descriptor, DescriptorParseError, DescriptorParseResult, Product};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::rc::Rc;

#[allow(unused)]
struct ProductInfo {
    prefix: String,
    product: Box<dyn Product>,
}

#[allow(unused)]
struct Registry {
    product_infos: Vec<Rc<ProductInfo>>,
}

#[allow(unused)]
impl Registry {
    fn new(product_infos: Vec<ProductInfo>) -> Self {
        Self {
            product_infos: product_infos.into_iter().map(Rc::new).collect::<Vec<_>>(),
        }
    }

    fn parse_descriptor(&self, s: &str) -> DescriptorParseResult<DescriptorInfo> {
        let Some((product_info, tail)) = self.find_product_info(s) else {
            return Err(DescriptorParseError::Other(anyhow!("unsupported descriptor format {s}")));
        };

        let descriptor = product_info.product.parse_descriptor(tail)?;
        Ok(DescriptorInfo {
            product_info: Rc::clone(product_info),
            descriptor,
        })
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

struct DescriptorInfo {
    product_info: Rc<ProductInfo>,
    descriptor: Box<dyn Descriptor>,
}

impl Display for DescriptorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.product_info.prefix, self.descriptor)
    }
}

#[cfg(test)]
mod tests {
    use super::{ProductInfo, Registry};
    use anyhow::Result;
    use isopy_openjdk::OpenJdk;
    use isopy_python::Python;
    use rstest::rstest;

    #[rstest]
    #[case("python:1.2.3:11223344", "1.2.3:11223344")]
    #[case("python:1.2.3:11223344", "python:1.2.3:11223344")]
    #[case("openjdk:19.0.1+10", "openjdk:19.0.1+10")]
    fn parse_descriptor(#[case] expected_str: &str, #[case] input: &str) -> Result<()> {
        let registry = Registry::new(vec![
            ProductInfo {
                prefix: String::from("python"),
                product: Box::<Python>::default(),
            },
            ProductInfo {
                prefix: String::from("openjdk"),
                product: Box::<OpenJdk>::default(),
            },
        ]);

        let descriptor_info = registry.parse_descriptor(input)?;
        assert_eq!(expected_str, descriptor_info.to_string());
        Ok(())
    }
}
