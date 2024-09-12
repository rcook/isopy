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
use crate::sanitize::{sanitize, sanitize_with_options, SanitizeOptions};
use anyhow::{bail, Result};
use url::Url;

#[derive(Debug, PartialEq)]
pub struct FileNamePartRefs<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
}

impl<'a> FileNamePartRefs<'a> {
    pub fn split(s: &'a str) -> Self {
        let Some(idx) = s.rfind('.') else {
            let (prefix, suffix) = s.split_at(s.len());
            return Self { prefix, suffix };
        };

        let (px0, sx0) = s.split_at(idx);

        if let Some(idx) = px0.rfind('.') {
            let (_, sx1) = px0.split_at(idx);
            if sx1 == ".tar" {
                let (prefix, suffix) = s.split_at(idx);
                return Self { prefix, suffix };
            }
        }

        Self {
            prefix: px0,
            suffix: sx0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FileNameParts {
    pub prefix: String,
    pub suffix: String,
}

impl FileNameParts {
    #[must_use]
    pub fn from_str_safe(s: &str) -> Self {
        let file_name_parts = FileNamePartRefs::split(s);
        let options = SanitizeOptions::default().retain_dots(true);
        Self {
            prefix: sanitize(file_name_parts.prefix),
            suffix: sanitize_with_options(file_name_parts.suffix, &options),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn from_url_safe(url: &Url) -> Result<Self> {
        if !Self::can_be_sanitized(url) {
            bail!("Url {url} cannot be sanitized")
        }

        let mut url_without_path = url.clone();
        url_without_path.set_path("");
        let mut prefix = sanitize(url_without_path.as_str());
        if prefix.ends_with('_') {
            prefix.pop();
        }

        let file_name_parts = Self::from_str_safe(url.path());
        assert!(file_name_parts.prefix.starts_with('_'));
        prefix.push_str(&file_name_parts.prefix);

        Ok(Self {
            prefix,
            suffix: file_name_parts.suffix,
        })
    }

    fn can_be_sanitized(url: &Url) -> bool {
        !url.cannot_be_a_base()
            && url.username().is_empty()
            && url.password().is_none()
            && url.port().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::tng::file_name_parts::{FileNamePartRefs, FileNameParts};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case("temp", ("temp", ""))]
    #[case("file.tar.gz", ("file", ".tar.gz"))]
    #[case("other.file.tar.gz", ("other.file", ".tar.gz"))]
    #[case("file.txt", ("file", ".txt"))]
    #[case("other.file.other.gz", ("other.file.other", ".gz"))]
    fn split(#[case] input: &str, #[case] expected: (&str, &str)) {
        assert_eq!(
            FileNamePartRefs {
                prefix: expected.0,
                suffix: expected.1
            },
            FileNamePartRefs::split(input)
        );
    }

    #[rstest]
    #[case("file", ("file", ""))]
    #[case("file&name.t&ar.g&z", ("file_name_t_ar", ".g_z"))]
    #[case("file&&name.tar.zst", ("file_name", ".tar.zst"))]
    fn from_str_safe(#[case] input: &str, #[case] expected: (&str, &str)) {
        assert_eq!(
            FileNameParts {
                prefix: String::from(expected.0),
                suffix: String::from(expected.1)
            },
            FileNameParts::from_str_safe(input)
        );
    }

    #[rstest]
    #[case("http://www.foo.com/file", ("http_www_foo_com_file", ""))]
    #[case("http://www.foo.com/file&name.t&ar.g&z", ("http_www_foo_com_file_name_t_ar", ".g_z"))]
    #[case("http://www.foo.com/file&&name.tar.zst", ("http_www_foo_com_file_name", ".tar.zst"))]
    fn from_url_safe(#[case] input: &str, #[case] expected: (&str, &str)) -> Result<()> {
        let url = input.parse()?;
        assert_eq!(
            FileNameParts {
                prefix: String::from(expected.0),
                suffix: String::from(expected.1)
            },
            FileNameParts::from_url_safe(&url)?
        );
        Ok(())
    }
}
