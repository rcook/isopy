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
use super::indicator::Indicator;
use super::state::State;
use std::borrow::Cow;
use std::sync::Arc;

pub struct Op {
    state: Arc<State>,
    indicator: Arc<Indicator>,
}

#[allow(unused)]
impl Op {
    pub fn set_position(&self, pos: u64) {
        self.indicator.set_position(pos)
    }

    pub fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        self.indicator.set_message(msg)
    }

    pub fn println(&self, msg: &str) {
        self.indicator.println(msg);
    }

    pub(crate) fn new(state: Arc<State>, indicator: Arc<Indicator>) -> Self {
        Self { state, indicator }
    }
}

impl Drop for Op {
    fn drop(&mut self) {
        // (TIME OF CHECK)
        match Arc::strong_count(&self.indicator) {
            2 => {
                // Indicator is still referenced by "state", so let's drop it
                // (TIME OF USE)
                drop(self.state.indicator.take());

                // TBD: There is a TOCTOU bug in this code! Another thread
                // could've jumped in and replaced self.state.indicator
                // between the time of check and the time of use (labelled
                // above). I don't know how much I care about this. This
                // assert will crash the program if we encounter this
                // condition. If I feel strongly motivated, I'll stick a
                // mutex around something.
                assert_eq!(
                    1,
                    Arc::strong_count(&self.indicator),
                    "there's a data race here which I'll need to think about"
                );
            }
            1 => {
                // Indicator is only referenced by this struct: do nothing
            }
            _ => unreachable!("this should never happen"),
        }
    }
}
