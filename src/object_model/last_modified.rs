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
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct LastModified(String);

impl LastModified {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for LastModified {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl FromStr for LastModified {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from(s)))
    }
}

impl TryFrom<&SystemTime> for LastModified {
    type Error = Error;

    fn try_from(value: &SystemTime) -> std::result::Result<Self, Self::Error> {
        let nanos = value.duration_since(UNIX_EPOCH)?.as_nanos();
        nanos.to_string().parse::<LastModified>()
    }
}

impl TryFrom<&LastModified> for SystemTime {
    type Error = Error;

    fn try_from(value: &LastModified) -> std::result::Result<Self, Self::Error> {
        let nanos = str::parse::<u64>(value.as_str())?;
        Ok(UNIX_EPOCH + Duration::from_nanos(nanos))
    }
}

#[cfg(test)]
mod tests {
    use super::LastModified;
    use anyhow::Result;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn tryfrom_basics() -> Result<()> {
        let system_time = UNIX_EPOCH + Duration::from_secs(12345678);
        let last_modified = LastModified::try_from(&system_time)?;
        assert_eq!("12345678000000000", format!("{}", last_modified));
        let result = SystemTime::try_from(&last_modified)?;
        assert_eq!(system_time, result);
        Ok(())
    }

    #[test]
    fn parse_basics() -> Result<()> {
        let last_modified = "last-modified".parse::<LastModified>()?;
        assert_eq!("last-modified", format!("{}", last_modified));
        Ok(())
    }
}
