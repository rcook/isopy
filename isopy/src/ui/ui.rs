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
use crate::ui::logger::Logger;
use crate::ui::op::Op;
use crate::ui::op::OpProgress;
use crate::ui::state::State;
use anyhow::Result;
use log::set_boxed_logger;
use std::sync::Arc;

pub struct Ui {
    state: Arc<State>,
}

impl Ui {
    pub fn new(enable_logger: bool) -> Result<Self> {
        let state = Arc::new(State::new());

        if enable_logger {
            set_boxed_logger(Box::new(Logger::new(Arc::clone(&state))))?;
        }

        Ok(Self { state })
    }

    pub fn begin_operation(&self, len: Option<OpProgress>) -> Result<Op> {
        Ok(Op::new(
            Arc::clone(&self.state),
            self.state.make_indicator(len)?,
        ))
    }

    #[allow(unused)]
    pub fn print(&self, msg: &str) {
        self.state.print(msg);
    }
}
