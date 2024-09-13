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
use crate::progress_indicator_options::ProgressIndicatorOptions;
use crate::Extent;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProgressIndicator(Option<Arc<ProgressBar>>);

impl ProgressIndicator {
    #[allow(unused)]
    pub fn new(options: &ProgressIndicatorOptions) -> Result<Self> {
        if !options.enabled {
            return Ok(Self(None));
        }

        let (progress_bar, template) = match &options.extent {
            Extent::Unknown => (
                ProgressBar::new_spinner(),
                "[{elapsed_precise:.green}]  {spinner:.cyan/blue}  {wide_msg:.yellow}",
            ),
            Extent::Bytes(len) => (
                ProgressBar::new(*len),
                "[{elapsed_precise:.green}]  {spinner:.cyan/blue}  [{eta_precise:.yellow} remaining]  {bar}  {decimal_bytes} of {decimal_total_bytes}  {wide_msg:.yellow}"
            ),
        };

        progress_bar.set_style(ProgressStyle::with_template(template)?);

        Ok(Self(Some(Arc::new(progress_bar))))
    }

    pub fn set_progress(&self, pos: u64) {
        if let Some(ref inner) = self.0 {
            inner.set_position(pos);
        }
    }

    pub fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        if let Some(ref inner) = self.0 {
            inner.set_message(msg);
        }
    }

    pub fn println<I: AsRef<str>>(&self, msg: I) {
        if let Some(ref inner) = self.0 {
            inner.println(msg);
        }
    }

    pub fn finish_and_clear(&self) {
        if let Some(ref inner) = self.0 {
            inner.finish_and_clear();
        }
    }
}

unsafe impl Send for ProgressIndicator {}
