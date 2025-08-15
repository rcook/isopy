// Copyright (c) 2025 Richard Cook
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
use anyhow::{Result, bail};
use chrono::{DateTime, Utc};
use reqwest::{Response, StatusCode};

pub fn error_for_github_rate_limit(response: &Response) -> Result<()> {
    if response.status() != StatusCode::FORBIDDEN {
        return Ok(());
    }

    let headers = response.headers();

    if headers.get("x-github-request-id").is_none() {
        return Ok(());
    }

    let Some(h) = headers.get("x-ratelimit-reset") else {
        return Ok(());
    };

    let Ok(s) = h.to_str() else { return Ok(()) };

    let Ok(reset_timestamp) = s.parse::<i64>() else {
        return Ok(());
    };

    let Some(reset_date_time) = DateTime::<Utc>::from_timestamp(reset_timestamp, 0) else {
        return Ok(());
    };

    let Some(h) = headers.get("x-ratelimit-remaining") else {
        return Ok(());
    };

    let Ok(s) = h.to_str() else { return Ok(()) };

    let Ok(value) = s.parse::<i32>() else {
        return Ok(());
    };

    if value != 0 {
        return Ok(());
    }

    bail!(
        "GitHub rate limit was exceeded (limit resets at {reset_date_time}): please try again later!"
    )
}
