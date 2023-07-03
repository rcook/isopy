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
use crate::constants::{OPENJDK_PRODUCT_VERSION_PREFIX, PYTHON_PRODUCT_VERSION_PREFIX};
use isopy_lib::DescriptorParseError;
use isopy_openjdk::OpenJdkDescriptor;
use isopy_python::PythonDescriptor;
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum ProductDescriptor {
    Python(PythonDescriptor),
    OpenJdk(OpenJdkDescriptor),
}

impl Display for ProductDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Python(d) => write!(f, "{PYTHON_PRODUCT_VERSION_PREFIX}:{d}"),
            Self::OpenJdk(d) => write!(f, "{OPENJDK_PRODUCT_VERSION_PREFIX}:{d}"),
        }
    }
}

impl FromStr for ProductDescriptor {
    type Err = DescriptorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(':') {
            Some((prefix, suffix)) if prefix == PYTHON_PRODUCT_VERSION_PREFIX => {
                Self::Python(suffix.parse::<PythonDescriptor>()?)
            }
            Some((prefix, suffix)) if prefix == OPENJDK_PRODUCT_VERSION_PREFIX => {
                Self::OpenJdk(suffix.parse::<OpenJdkDescriptor>()?)
            }
            _ => Self::OpenJdk(s.parse::<OpenJdkDescriptor>()?),
        })
    }
}

impl<'de> Deserialize<'de> for ProductDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::ProductDescriptor;
    use anyhow::Result;
    use isopy_openjdk::{OpenJdkDescriptor, OpenJdkVersion};
    use isopy_python::{PythonDescriptor, PythonVersion, Tag};
    use rstest::rstest;

    #[rstest]
    #[case(
        ProductDescriptor::Python(PythonDescriptor { version: PythonVersion::new(111, 222, 333), tag: None}),
        "111.222.333"
    )]
    #[case(
        ProductDescriptor::Python(PythonDescriptor { version: PythonVersion::new(111, 222, 333), tag: None}),
        "python:111.222.333"
    )]
    #[case(
        ProductDescriptor::Python(PythonDescriptor { version: PythonVersion::new(111, 222, 333), tag: Some("tag".parse::<Tag>().expect("test: must be valid tag"))}),
        "111.222.333:tag"
    )]
    #[case(
        ProductDescriptor::Python(PythonDescriptor { version: PythonVersion::new(111, 222, 333), tag: Some("tag".parse::<Tag>().expect("test: must be valid tag"))}),
        "python:111.222.333:tag"
    )]
    #[case(
        ProductDescriptor::OpenJdk(OpenJdkDescriptor { version: "111.222.333+444".parse::<OpenJdkVersion>().expect("test: must be valid OpenJDK version")}),
        "openjdk:111.222.333+444"
    )]
    fn parse(#[case] expected_result: ProductDescriptor, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse::<ProductDescriptor>()?);
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case("111")]
    fn parse_errors(#[case] input: &str) {
        assert!(input.parse::<ProductDescriptor>().is_err());
    }
}
