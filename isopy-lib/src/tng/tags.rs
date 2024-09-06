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
pub struct Tags {
    tags: Vec<String>,
    default_tags: Vec<String>,
    other_tags: Vec<String>,
}

impl Tags {
    pub fn new(tags: Vec<String>, default_tags: Vec<String>, other_tags: Vec<String>) -> Self {
        Self {
            tags,
            default_tags,
            other_tags,
        }
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn default_tags(&self) -> &Vec<String> {
        &self.default_tags
    }

    pub fn other_tags(&self) -> &Vec<String> {
        &self.other_tags
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self {
            tags: vec![],
            default_tags: vec![],
            other_tags: vec![],
        }
    }
}
