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
use super::product_descriptor::ProductDescriptor;
use super::product_info::ProductInfo;
use isopy_lib::{Descriptor, DescriptorParseResult};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::rc::Rc;

pub struct DescriptorInfo {
    pub product_info: Rc<ProductInfo>,
    pub descriptor: Box<dyn Descriptor>,
}

impl DescriptorInfo {
    pub fn to_product_descriptor(&self) -> DescriptorParseResult<ProductDescriptor> {
        // Temporary hack: should be replaced with calls into
        // implementers of Descriptor trait etc.
        self.descriptor.to_string().parse::<ProductDescriptor>()
    }
}

impl Display for DescriptorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.product_info.prefix, self.descriptor)
    }
}
