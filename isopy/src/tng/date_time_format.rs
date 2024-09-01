use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub(crate) fn serialize<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{}", date_time.format(FORMAT)))
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naive_date_time =
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    let date_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_date_time, Utc);
    Ok(date_time)
}
