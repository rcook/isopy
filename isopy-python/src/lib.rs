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
#![warn(clippy::all)]
//#![warn(clippy::cargo)]
//#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
//#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::future_not_send)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::option_if_let_else)]
mod python_product_descriptor;
mod python_version;
mod tag;

pub use self::python_product_descriptor::PythonProductDescriptor;
pub use self::python_version::PythonVersion;
pub use self::tag::{option_tag, Tag};

use isopy_lib::{Descriptor, DescriptorParseError, DescriptorParseResult, Product};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;
use thiserror::Error;

const NAME: &str = "Python";

pub struct Python;

impl Default for Python {
    fn default() -> Self {
        Self
    }
}

impl Product for Python {
    fn name(&self) -> &str {
        NAME
    }

    fn parse_descriptor(&self, s: &str) -> DescriptorParseResult<Box<dyn Descriptor>> {
        Ok(Box::new(
            s.parse::<PythonDescriptor>()
                .map_err(DescriptorParseError::other)?,
        ))
    }
}

#[derive(Debug, Error)]
pub enum PythonDescriptorParseError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct PythonDescriptor {
    version: String,
}

impl FromStr for PythonDescriptor {
    type Err = PythonDescriptorParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self {
            version: String::from(s),
        })
    }
}

impl Display for PythonDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.version)
    }
}

impl Descriptor for PythonDescriptor {}
