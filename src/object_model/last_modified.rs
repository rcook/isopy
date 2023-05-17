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
use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct LastModified(String);

impl LastModified {
    pub fn parse<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for LastModified {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&SystemTime> for LastModified {
    type Error = AnyhowError;

    fn try_from(value: &SystemTime) -> StdResult<Self, Self::Error> {
        let nanos = value.duration_since(UNIX_EPOCH)?.as_nanos();
        Ok(LastModified::parse(format!("{}", nanos)))
    }
}

impl TryFrom<&LastModified> for SystemTime {
    type Error = AnyhowError;

    fn try_from(value: &LastModified) -> StdResult<Self, Self::Error> {
        let nanos = str::parse::<u64>(value.as_str())?;
        Ok(UNIX_EPOCH + Duration::from_nanos(nanos))
    }
}
