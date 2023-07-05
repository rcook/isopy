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
use crate::plugin::Plugin;
use crate::registry::Registry;
use isopy_lib::{Descriptor, ParseDescriptorError};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DescriptorId {
    descriptor_info: DescriptorInfo,
}

impl DescriptorId {
    pub fn plugin(&self) -> &Plugin {
        &self.descriptor_info.plugin
    }

    pub fn descriptor(&self) -> &dyn Descriptor {
        self.descriptor_info.descriptor.as_ref().as_ref()
    }
}

impl FromStr for DescriptorId {
    type Err = ParseDescriptorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let descriptor_info = Registry::global().parse_descriptor(s)?;
        Ok(Self { descriptor_info })
    }
}

impl Display for DescriptorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.descriptor_info)
    }
}

impl<'de> Deserialize<'de> for DescriptorId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Self>()
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for DescriptorId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.descriptor_info.to_string())
    }
}
