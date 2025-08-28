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
use anyhow::{Error, Result, anyhow};
use reqwest::Response;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub struct LinkHeader {
    pub next: Option<Url>,
    _last: Option<Url>,
    _links: HashMap<String, String>,
}

impl LinkHeader {
    pub fn from_response(response: &Response) -> Result<Option<Self>> {
        let Some(link_header) = response.headers().get("link") else {
            return Ok(None);
        };

        let s = link_header.to_str().map_err(|e| anyhow!(e))?;
        Ok(Some(s.parse::<Self>()?))
    }

    fn parse_link_header(s: &str) -> HashMap<String, String> {
        fn parse_url_part(s: &str) -> Option<String> {
            s.strip_prefix('<')
                .and_then(|s0| s0.strip_suffix('>'))
                .map(std::string::ToString::to_string)
        }

        fn parse_rel_part(s: &str) -> Option<String> {
            s.strip_prefix("rel=\"")
                .and_then(|s0| s0.strip_suffix('"'))
                .map(std::string::ToString::to_string)
        }

        s.split(',')
            .filter_map(|part| {
                part.split_once(';').and_then(|(u, r)| {
                    parse_url_part(u.trim())
                        .and_then(|u0| parse_rel_part(r.trim()).map(|r0| (r0, u0)))
                })
            })
            .collect::<HashMap<_, _>>()
    }

    fn get_link_url(links: &HashMap<String, String>, k: &str) -> Result<Option<Url>> {
        let Some(s) = links.get(k) else {
            return Ok(None);
        };

        Ok(Some(s.parse::<Url>()?))
    }
}

impl FromStr for LinkHeader {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let links = Self::parse_link_header(s);
        let next = Self::get_link_url(&links, "next")?;
        let last = Self::get_link_url(&links, "last")?;
        Ok(Self {
            next,
            _last: last,
            _links: links,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::link_header::LinkHeader;
    use anyhow::Result;
    use reqwest::Url;

    #[test]
    fn basics() -> Result<()> {
        let result = "<https://api.adoptium.net/v3/info/release_versions?heap_size=normal&image_type=jdk&project=jdk&release_type=ga&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse&page=1&page_size=10>; rel=\"next\"".parse::<LinkHeader>()?;
        assert_eq!(Some("https://api.adoptium.net/v3/info/release_versions?heap_size=normal&image_type=jdk&project=jdk&release_type=ga&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse&page=1&page_size=10".parse::<Url>()?), result.next);
        assert!(result._last.is_none());
        assert_eq!(1, result._links.len());
        Ok(())
    }
}
