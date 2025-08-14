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
use crate::extent::Extent;
use crate::package_manager::InstallPackageOptions;
use crate::progress_indicator::{ProgressIndicator, ProgressIndicatorOptionsBuilder};
use anyhow::{Result, bail};
use decompress::{ExtractOptsBuilder, decompress};
use log::info;
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum ArchiveType {
    TarGz,
    TarZst,
    Zip,
}

impl ArchiveType {
    #[must_use]
    pub fn strip_suffix(s: &str) -> Option<(Self, &str)> {
        for value in Self::iter() {
            if let Some(prefix) = s.strip_suffix(value.suffix()) {
                return Some((value, prefix));
            }
        }
        None
    }

    #[must_use]
    pub const fn suffix(&self) -> &str {
        match self {
            Self::TarGz => ".tar.gz",
            Self::TarZst => ".tar.zst",
            Self::Zip => ".zip",
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn unpack(
        &self,
        archive_path: &Path,
        dir: &Path,
        options: &InstallPackageOptions,
    ) -> Result<()> {
        if dir.exists() {
            bail!("Output directory {} already exists", dir.display())
        }

        let progress_indicator = ProgressIndicator::new(
            &ProgressIndicatorOptionsBuilder::default()
                .enabled(options.show_progress)
                .extent(Extent::Unknown)
                .build()?,
        )?;

        {
            let progress_indicator = progress_indicator.clone();
            let options = ExtractOptsBuilder::default()
                .strip(1)
                .filter(move |args| {
                    progress_indicator.set_message(format!("Unpacking {}", args.rel_path()));
                    true
                })
                .build()?;
            decompress(archive_path, dir, &options)?;
        }

        progress_indicator.finish_and_clear();

        info!("Unpacked package to {}", dir.display());
        Ok(())
    }
}
