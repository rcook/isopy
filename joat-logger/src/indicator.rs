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
use crate::op::OpProgress;
use crate::result::Result;
use indicatif::{ProgressBar, ProgressStyle};
use uuid::Uuid;

pub struct Indicator {
    id: Uuid,
    progress_bar: ProgressBar,
}

impl Indicator {
    pub(crate) fn new(len: Option<OpProgress>) -> Result<Self> {
        let (progress_bar, template) = match len {
            Some(n) => (
                ProgressBar::new(n),
                "[{elapsed_precise:.green}]  {spinner:.cyan/blue}  {pos:>7}  {wide_msg:.yellow}",
            ),
            None => (
                ProgressBar::new_spinner(),
                "[{elapsed_precise:.green}]  {spinner:.cyan/blue}           {wide_msg:.yellow}",
            ),
        };

        progress_bar.set_style(
            ProgressStyle::with_template(template)
                .map_err(|_| Error::CouldNotConfigureProgressBar)?,
        );

        Ok(Self {
            id: Uuid::new_v4(),
            progress_bar,
        })
    }

    pub(crate) const fn id(&self) -> &Uuid {
        &self.id
    }

    pub(crate) fn set_progress(&self, value: OpProgress) {
        self.progress_bar.set_position(value);
    }

    pub(crate) fn set_message(&self, s: &str) {
        self.progress_bar.set_message(String::from(s));
    }

    pub(crate) fn print(&self, s: &str) {
        self.progress_bar.println(s);
    }
}
