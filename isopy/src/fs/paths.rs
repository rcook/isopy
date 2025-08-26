// The MIT License (MIT)
//
// Copyright (c) 2020-3 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use chrono::{DateTime, SecondsFormat, Utc};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[must_use]
pub fn label_file_name(path: &Path, label: &str) -> Option<PathBuf> {
    let mut file_name = OsString::new();

    if let Some(s) = path.file_stem() {
        file_name.push(s);
    } else {
        return None;
    }

    file_name.push("-");
    file_name.push(label);

    if let Some(s) = path.extension() {
        file_name.push(".");
        file_name.push(s);
    }

    Some(path.with_file_name(file_name))
}

#[must_use]
pub fn file_name_safe_timestamp(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
        .replace(['-', ':', '.'], "")
}

#[cfg(test)]
mod tests {
    use crate::fs::paths::{file_name_safe_timestamp, label_file_name};
    use chrono::{TimeZone, Utc};
    use rstest::rstest;
    use std::path::PathBuf;

    #[rstest]
    #[case(Some(PathBuf::from("/aaa/bbb/ccc-ddd.txt")), "/aaa/bbb/ccc.txt", "ddd")]
    #[case(Some(PathBuf::from("/aaa/bbb/ccc-ddd")), "/aaa/bbb/ccc", "ddd")]
    #[case(Some(PathBuf::from("ccc-ddd.txt")), "ccc.txt", "ddd")]
    #[case(Some(PathBuf::from("ccc-ddd")), "ccc", "ddd")]
    fn label_file_name_basics(
        #[case] expected_path: Option<PathBuf>,
        #[case] path: PathBuf,
        #[case] label: &str,
    ) {
        assert_eq!(expected_path, label_file_name(&path, label));
    }

    #[test]
    fn file_name_safe_timestamp_basics() {
        let dt = Utc
            .with_ymd_and_hms(2019, 3, 17, 16, 43, 0)
            .single()
            .expect("must be valid");
        assert_eq!("20190317T164300000Z", file_name_safe_timestamp(&dt));
    }
}
