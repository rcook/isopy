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
use std::borrow::Cow;
use url::Url;

#[must_use]
pub fn file_url(url: &Url) -> Cow<Url> {
    helper(url, false)
}

#[must_use]
pub fn dir_url(url: &Url) -> Cow<Url> {
    helper(url, true)
}

fn helper(url: &Url, slash: bool) -> Cow<Url> {
    let path_str = url.path();
    let trimmed = path_str.trim_end_matches(&['/']);
    if !slash && trimmed == path_str {
        return Cow::Borrowed(url);
    }

    let mut s = String::from(trimmed);
    if slash {
        s.push('/');
    }

    let mut url = url.clone();
    url.set_path(&s);
    Cow::Owned(url)
}

#[cfg(test)]
mod tests {
    use super::helper;
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
    fn helper_basics_no_slash(#[case] expected_output: &str, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_output, helper(&Url::parse(input)?, false).as_str());
        Ok(())
    }

    #[rstest]
    #[case("https://aaa/", "https://aaa")]
    #[case("https://aaa/", "https://aaa/")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc/")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc//")]
    #[case("https://aaa/bbb/ccc/", "https://aaa/bbb/ccc///")]
    fn helper_basics_slash(#[case] expected_output: &str, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_output, helper(&Url::parse(input)?, true).as_str());
        Ok(())
    }
}
