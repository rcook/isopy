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
use super::state::State;
use std::borrow::Cow;
use std::sync::Arc;

pub struct Op {
    state: Arc<State>,
}

#[allow(unused)]
impl Op {
    pub fn set_position(&self, pos: u64) {
        if let Some(indicator) = self.state.indicator.borrow().as_ref() {
            indicator.set_position(pos);
        } else {
            panic!("INDICATOR INVALID")
        }
    }

    pub fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        if let Some(indicator) = self.state.indicator.borrow().as_ref() {
            indicator.set_message(msg);
        } else {
            panic!("INDICATOR INVALID")
        }
    }

    pub fn println(&self, msg: &str) {
        if let Some(indicator) = self.state.indicator.borrow().as_ref() {
            indicator.println(msg);
        } else {
            panic!("INDICATOR INVALID")
        }
    }

    pub(crate) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

impl Drop for Op {
    fn drop(&mut self) {
        drop(self.state.indicator.take());
    }
}
