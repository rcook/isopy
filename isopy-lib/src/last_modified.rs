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
use crate::{serializable_newtype, TryToString};
use anyhow::{bail, Error};
use std::result::Result as StdResult;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type NanosType = u128;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum LastModifiedImpl {
    Nanos(NanosType),
    UnknownFormat(String),
}

serializable_newtype!(LastModified, LastModifiedImpl);

impl FromStr for LastModified {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self(if let Ok(nanos) = s.parse::<NanosType>() {
            LastModifiedImpl::Nanos(nanos)
        } else {
            LastModifiedImpl::UnknownFormat(String::from(s))
        }))
    }
}

impl TryToString for LastModified {
    fn to_string_lossy(&self) -> String {
        match &self.0 {
            LastModifiedImpl::Nanos(nanos) => nanos.to_string(),
            LastModifiedImpl::UnknownFormat(s) => s.clone(),
        }
    }

    fn try_to_string(&self) -> Option<String> {
        Some(self.to_string_lossy())
    }
}

impl TryFrom<&SystemTime> for LastModified {
    type Error = Error;

    fn try_from(value: &SystemTime) -> StdResult<Self, Self::Error> {
        Ok(Self(LastModifiedImpl::Nanos(
            value.duration_since(UNIX_EPOCH)?.as_nanos(),
        )))
    }
}

impl TryFrom<&LastModified> for SystemTime {
    type Error = Error;

    fn try_from(value: &LastModified) -> StdResult<Self, Self::Error> {
        match &value.0 {
            LastModifiedImpl::Nanos(nanos) => {
                let nanos_i64 = (*nanos).try_into()?;
                Ok(UNIX_EPOCH + Duration::from_nanos(nanos_i64))
            }
            LastModifiedImpl::UnknownFormat(s) => bail!("cannot convert {s} to system time"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LastModified;
    use anyhow::Result;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn try_from_basics() -> Result<()> {
        let system_time = UNIX_EPOCH + Duration::from_secs(12_345_678);
        let last_modified = LastModified::try_from(&system_time)?;
        assert_eq!("12345678000000000", format!("{last_modified}"));
        let result = SystemTime::try_from(&last_modified)?;
        assert_eq!(system_time, result);
        Ok(())
    }

    #[test]
    fn parse_basics() -> Result<()> {
        let last_modified = "last-modified".parse::<LastModified>()?;
        assert_eq!("last-modified", format!("{last_modified}"));
        Ok(())
    }
}
