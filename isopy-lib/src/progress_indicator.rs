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
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::rc::Rc;

#[derive(Clone)]
pub struct ProgressIndicator(Option<Rc<ProgressBar>>);

impl ProgressIndicator {
    #[allow(unused)]
    pub fn new(show_progress: bool) -> Result<Self> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_style(ProgressStyle::with_template(
            "[{elapsed_precise:.green}]  {spinner:.cyan/blue}  {pos:>7}  {wide_msg:.yellow}",
        )?);

        Ok(if show_progress {
            Self(Some(Rc::new(progress_bar)))
        } else {
            Self(None)
        })
    }

    #[allow(unused)]
    pub fn new_spinner(show_progress: bool) -> Result<Self> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_style(ProgressStyle::with_template(
            "[{elapsed_precise:.green}]  {spinner:.cyan/blue}           {wide_msg:.yellow}",
        )?);

        Ok(if show_progress {
            Self(Some(Rc::new(progress_bar)))
        } else {
            Self(None)
        })
    }

    pub fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        if let Some(inner) = &self.0 {
            inner.set_message(msg);
        }
    }

    pub fn finish(&self) {
        if let Some(inner) = &self.0 {
            inner.finish_and_clear();
        }
    }
}
