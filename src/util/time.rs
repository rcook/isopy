use crate::object_model::LastModified;
use crate::result::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn to_last_modified(time: &SystemTime) -> Result<LastModified> {
    let nanos = time.duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(LastModified::parse(format!("{}", nanos)))
}

pub fn to_system_time(last_modified: &LastModified) -> Result<SystemTime> {
    let nanos = str::parse::<u64>(last_modified.as_str())?;
    Ok(UNIX_EPOCH + Duration::from_nanos(nanos))
}
