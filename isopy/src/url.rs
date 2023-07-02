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
use reqwest::Url;

pub fn file_url(url: &Url) -> Url {
    helper(url, false)
}

pub fn dir_url(url: &Url) -> Url {
    helper(url, true)
}

fn helper(url: &Url, add_slash: bool) -> Url {
    let mut s = String::from(url.path().trim_end_matches(&['/']));
    if add_slash {
        s.push('/');
    }

    let mut new_url = url.clone();
    new_url.set_path(&s);
    new_url
}

#[cfg(test)]
mod tests {
    use super::helper;
    use anyhow::Result;
    use reqwest::Url;
    use rstest::rstest;

    #[rstest]
    #[case("https://aaa/", "https://aaa")]
    #[case("https://aaa/", "https://aaa/")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc/")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc//")]
    #[case("https://aaa/bbb/ccc", "https://aaa/bbb/ccc///")]
    fn helper_basics_no_add_slash(
        #[case] expected_output: &str,
        #[case] input: &str,
    ) -> Result<()> {
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
    fn helper_basics_add_slash(#[case] expected_output: &str, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_output, helper(&Url::parse(input)?, true).as_str());
        Ok(())
    }
}
