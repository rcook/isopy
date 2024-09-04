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
use crate::tng::accept::Accept;
use crate::tng::checksum::Checksum;

pub struct DownloadOptions {
    pub accept: Option<Accept>,
    pub update: bool,
    pub checksum: Option<Checksum>,
}

impl DownloadOptions {
    pub fn json() -> Self {
        Self::default().accept(Some(Accept::ApplicationJson))
    }

    pub fn accept(mut self, value: Option<Accept>) -> Self {
        self.accept = value;
        self
    }

    pub fn update(mut self, value: bool) -> Self {
        self.update = value;
        self
    }

    pub fn checksum(mut self, value: Option<Checksum>) -> Self {
        self.checksum = value;
        self
    }
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            accept: None,
            update: false,
            checksum: None,
        }
    }
}