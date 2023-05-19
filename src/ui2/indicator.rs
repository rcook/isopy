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
use super::result::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;

pub type IndicatorLength = u64;

pub(crate) struct Indicator {
    progress_bar: ProgressBar,
}

impl Indicator {
    pub(crate) fn new(len: Option<IndicatorLength>) -> Result<Self> {
        Ok(Self {
            progress_bar: match len {
                Some(n) => {
                    let p = ProgressBar::new(n);

                    let progress_style = ProgressStyle::with_template(
                    "[{elapsed_precise:.green}]  {spinner:.cyan/blue}  {pos:>3}%  {wide_msg:.yellow}",
                )
                .map_err(|_| Error::CouldNotConfigureProgressBar)?;

                    p.set_style(progress_style);
                    p
                }
                None => {
                    let p = ProgressBar::new_spinner();

                    let progress_style = ProgressStyle::with_template(
                    "[{elapsed_precise:.green}]  {spinner:.cyan/blue}        {wide_msg:.yellow}",
                )
                .map_err(|_| Error::CouldNotConfigureProgressBar)?;

                    p.set_style(progress_style);
                    p
                }
            },
        })
    }

    pub(crate) fn set_position(&self, pos: u64) {
        self.progress_bar.set_position(pos);
    }

    pub(crate) fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        self.progress_bar.set_message(msg)
    }

    pub(crate) fn println(&self, msg: &str) {
        self.progress_bar.println(msg)
    }
}

impl Drop for Indicator {
    fn drop(&mut self) {
        self.progress_bar.finish_and_clear();
    }
}
