use crate::result::Result;
use crate::util::ContentLength;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;

pub struct Indicator {
    progress_bar: ProgressBar,
}

impl Indicator {
    pub fn new(size: Option<ContentLength>) -> Result<Self> {
        let progress_bar = match size {
            None => ProgressBar::new_spinner(),
            Some(s) => ProgressBar::new(s),
        };
        progress_bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )?
            .progress_chars("##-"),
        );
        Ok(Self { progress_bar })
    }

    pub fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        self.progress_bar.set_message(msg)
    }

    pub fn set_position(&self, value: ContentLength) {
        self.progress_bar.set_position(value)
    }

    pub fn finish(&self) {
        self.progress_bar.finish()
    }
}
