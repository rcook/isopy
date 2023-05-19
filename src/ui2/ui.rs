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
use super::error::Error;
use super::indicator::{Indicator, IndicatorLength};
use super::logger::Logger;
use super::op::Op;
use super::options::Options;
use super::result::Result;
use super::state::State;
use std::sync::Arc;

pub struct Ui {
    state: Arc<State>,
}

#[allow(unused)]
impl Ui {
    pub fn new(options: &Options) -> Result<Self> {
        let state = Arc::new(State::new());
        let this = Self { state };
        this.set_options(options)?;
        Ok(this)
    }

    pub fn operation(&self, len: Option<IndicatorLength>) -> Result<Op> {
        drop(self.state.indicator.take());
        self.state.indicator.replace(Some(Indicator::new(len)?));
        Ok(Op::new(Arc::clone(&self.state)))
    }

    pub(crate) fn set_options(&self, options: &Options) -> Result<()> {
        if let Some(logger_options) = &options.logger {
            let mut owns_logger = self.state.owns_logger.borrow_mut();
            if !*owns_logger {
                Logger::init(Arc::clone(&self.state))?;
                *owns_logger = true;
            }

            Logger::set_max_level(logger_options.level);
        } else {
            if *self.state.owns_logger.borrow() {
                return Err(Error::CannotUnsetLogger);
            }
        }

        Ok(())
    }
}
