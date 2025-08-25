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
use crate::indicator::Indicator;
use crate::op::OpProgress;
use crate::result::Result;
use std::sync::{Arc, RwLock};

pub struct State {
    indicator: RwLock<Option<Arc<Indicator>>>,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            indicator: RwLock::new(None),
        }
    }

    pub(crate) fn make_indicator(&self, len: Option<OpProgress>) -> Result<Arc<Indicator>> {
        let mut writer = self.indicator.write().expect("lock is poisoned");
        *writer = None;
        let indicator = Arc::new(Indicator::new(len)?);
        *writer = Some(Arc::clone(&indicator));
        drop(writer);
        Ok(indicator)
    }

    pub(crate) fn release_indicator(&self, indicator: &Arc<Indicator>) {
        let mut writer = self.indicator.write().expect("lock is poisoned");
        if let Some(i) = &*writer
            && indicator.id() == i.id()
        {
            *writer = None;
        }
    }

    pub(crate) fn print(&self, s: &str) {
        if let Some(i) = &*self.indicator.read().expect("lock is poisoned") {
            i.print(s);
        } else {
            println!("{s}");
        }
    }
}
