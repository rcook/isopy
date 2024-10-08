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
const TRUE_LITERALS: [&str; 6] = ["y", "yes", "t", "true", "on", "1"];
const FALSE_LITERALS: [&str; 6] = ["n", "no", "f", "false", "off", "0"];

pub(crate) fn str_to_bool(s: &str) -> Option<bool> {
    let s = s.trim().to_lowercase();
    let t = s.as_str();
    if TRUE_LITERALS.contains(&t) {
        Some(true)
    } else if FALSE_LITERALS.contains(&t) {
        Some(false)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::bool_util::str_to_bool;
    use rstest::rstest;

    #[rstest]
    #[case(None, "junk")]
    #[case(None, "")]
    #[case(Some(true), "1")]
    #[case(Some(true), " 1 ")]
    #[case(Some(false), " 0 ")]
    fn basics(#[case] expected_result: Option<bool>, #[case] input: &str) {
        assert_eq!(expected_result, str_to_bool(input));
    }
}
