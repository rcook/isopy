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
use anyhow::Error;
use std::result::Result as StdResult;
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug)]
pub struct FileUrl(Url);

impl FileUrl {
    #[must_use]
    pub const fn as_url(&self) -> &Url {
        &self.0
    }
}

impl FromStr for FileUrl {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(s.parse::<Url>()?.into())
    }
}

impl From<Url> for FileUrl {
    fn from(value: Url) -> Self {
        let path_str = value.path().trim_end_matches('/');
        let mut url = value.clone();
        url.set_path(path_str);
        Self(url)
    }
}

impl From<FileUrl> for Url {
    fn from(value: FileUrl) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct DirUrl(Url);

impl DirUrl {
    #[must_use]
    pub const fn as_url(&self) -> &Url {
        &self.0
    }
}

impl FromStr for DirUrl {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut s = s.trim_end_matches('/').to_owned();
        s.push('/');
        Ok(Self(s.parse()?))
    }
}

impl From<Url> for DirUrl {
    fn from(value: Url) -> Self {
        let path_str = value.path().trim_end_matches('/');
        let mut path_str = path_str.to_owned();
        path_str.push('/');
        let mut url = value.clone();
        url.set_path(&path_str);
        Self(url)
    }
}

impl From<DirUrl> for Url {
    fn from(value: DirUrl) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::FileUrl;
    use crate::url::DirUrl;
    use anyhow::Result;
    use rstest::rstest;
    use url::Url;

    #[rstest]
    #[case("https://aaa/", "https://aaa")]
    #[case("https://aaa/", "https://aaa/")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc/")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc//")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc///")]
    fn file_url(#[case] expected_output: &str, #[case] input: &str) -> Result<()> {
        // Test parsing
        let file_url = input.parse::<FileUrl>()?;
        assert_eq!(expected_output, Into::<Url>::into(file_url).as_str());

        // Test conversion from Url
        let file_url: FileUrl = input.parse::<Url>()?.into();
        assert_eq!(expected_output, file_url.as_url().as_str());
        assert_eq!(expected_output, Into::<Url>::into(file_url).as_str());

        Ok(())
    }

    #[rstest]
    #[case("https://aaa/", "https://aaa")]
    #[case("https://aaa/", "https://aaa/")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc/")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc//")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc///")]
    fn dir_url(#[case] expected_output: &str, #[case] input: &str) -> Result<()> {
        // Test parsing
        let dir_url = input.parse::<DirUrl>()?;
        assert_eq!(expected_output, Into::<Url>::into(dir_url).as_str());

        // Test conversion from Url
        let dir_url: DirUrl = input.parse::<Url>()?.into();
        assert_eq!(expected_output, dir_url.as_url().as_str());
        assert_eq!(expected_output, Into::<Url>::into(dir_url).as_str());

        Ok(())
    }
}
