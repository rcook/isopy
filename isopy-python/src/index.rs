// Copyright (c) 2024 Richard Cook
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
use crate::item::Item;
use anyhow::Error;
use serde_json::Value;
use std::iter::from_fn;
use std::result::Result as StdResult;
use std::str::FromStr;

pub(crate) struct Index {
    value: Value,
    empty_items: Vec<Value>,
}

impl Index {
    pub(crate) fn new(value: Value) -> Self {
        Self {
            value,
            empty_items: Vec::new(),
        }
    }

    pub(crate) fn items(&self) -> impl Iterator<Item = Item<'_>> {
        let mut iter = self
            .value
            .as_array()
            .unwrap_or(&self.empty_items)
            .into_iter();
        from_fn(move || iter.next().map(|value| Item::new(value)))
    }
}

impl FromStr for Index {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self::new(serde_json::from_str(&s)?))
    }
}
