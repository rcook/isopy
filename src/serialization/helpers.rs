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
use crate::object_model::{LastModified, RepositoryName, Tag, Version};
use reqwest::Url;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_last_modified<S>(x: &LastModified, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_last_modified<'de, D>(deserializer: D) -> Result<LastModified, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<LastModified>()
        .map_err(Error::custom)
}

pub fn deserialize_repository_name<'de, D>(deserializer: D) -> Result<RepositoryName, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<RepositoryName>()
        .map_err(Error::custom)
}

pub fn serialize_repository_name<S>(x: &RepositoryName, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_tag<'de, D>(deserializer: D) -> Result<Tag, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<Tag>()
        .map_err(Error::custom)
}

pub fn serialize_tag<S>(x: &Tag, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_tag_opt<'de, D>(deserializer: D) -> Result<Option<Tag>, D::Error>
where
    D: Deserializer<'de>,
{
    let s_opt: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s_opt {
        Ok(Some(s.parse::<Tag>().map_err(Error::custom)?))
    } else {
        Ok(None)
    }
}

pub fn serialize_tag_opt<S>(tag: &Option<Tag>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(ref t) = *tag {
        return s.serialize_str(t.as_str());
    }
    s.serialize_none()
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Url::parse(&s).map_err(Error::custom)
}

pub fn serialize_url<S>(x: &Url, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}

pub fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<Version>()
        .map_err(Error::custom)
}

pub fn serialize_version<S>(x: &Version, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.as_str())
}
