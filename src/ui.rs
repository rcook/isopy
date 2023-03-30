use crate::error::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::time::Duration;

pub struct ProgressIndicator {
    progress_bar: ProgressBar,
}

impl ProgressIndicator {
    pub fn new(size_opt: Option<u64>) -> Result<Self> {
        match size_opt {
            Some(size) => {
                let progress_bar = ProgressBar::new(size);
                progress_bar.set_style(ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
                    .progress_chars("#>-"));
                Ok(Self {
                    progress_bar: progress_bar,
                })
            }
            None => {
                let progress_bar = ProgressBar::new_spinner();
                progress_bar.enable_steady_tick(Duration::from_millis(80));
                progress_bar.set_style(
                    ProgressStyle::with_template("{spinner:.blue} {msg}")
                        .unwrap()
                        // For more spinners check out the cli-spinners project:
                        // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                        .tick_strings(&[
                            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]",
                            "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]",
                            "[=   ]",
                        ]),
                );
                Ok(Self {
                    progress_bar: progress_bar,
                })
            }
        }
    }

    pub fn set_message(&self, msg: impl Into<Cow<'static, str>>) {
        self.progress_bar.set_message(msg)
    }

    pub fn set_position(&self, pos: u64) {
        self.progress_bar.set_position(pos);
    }

    pub fn finish_with_message(&self, msg: impl Into<Cow<'static, str>>) {
        self.progress_bar.finish_with_message(msg);
    }
}
