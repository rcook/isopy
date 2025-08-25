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
use crate::error::Error;
use crate::logger::Logger;
use crate::op::Op;
use crate::op::OpProgress;
use crate::result::Result;
use crate::state::State;
use log::set_boxed_logger;
use std::sync::Arc;

pub struct Ui {
    state: Arc<State>,
}

impl Ui {
    #[allow(unused)]
    pub fn new(enable_logger: bool) -> Result<Self> {
        let state = Arc::new(State::new());

        if enable_logger {
            set_boxed_logger(Box::new(Logger::new(Arc::clone(&state))))
                .map_err(|_| Error::CouldNotSetLogger)?;
        }

        Ok(Self { state })
    }

    #[allow(unused)]
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
