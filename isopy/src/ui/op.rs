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
use crate::ui::indicator::Indicator;
use crate::ui::state::State;
use std::sync::Arc;

pub type OpProgress = u64;

pub struct Op {
    state: Arc<State>,
    indicator: Arc<Indicator>,
}

impl Op {
    #[allow(unused)]
    pub fn set_progress(&self, value: OpProgress) {
        self.indicator.set_progress(value);
    }

    #[allow(unused)]
    pub fn set_message(&self, s: &str) {
        self.indicator.set_message(s);
    }

    #[allow(unused)]
    pub fn print(&self, s: &str) {
        self.indicator.print(s);
    }

    pub(crate) const fn new(state: Arc<State>, indicator: Arc<Indicator>) -> Self {
        Self { state, indicator }
    }
}

impl Drop for Op {
    fn drop(&mut self) {
        self.state.release_indicator(&self.indicator);
    }
}
