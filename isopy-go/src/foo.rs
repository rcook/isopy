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
use crate::download::download_index;
use crate::error::other_error;
use crate::filter::Filter;
use crate::go_version::GoVersion;
use crate::result::IsopyGoResult;
use crate::serialization::IndexRec;
use joatmon::read_yaml_file;
use std::path::Path;

pub async fn hello(cache_dir: &Path) -> IsopyGoResult<()> {
    let index_path = cache_dir.join("go.yaml");

    download_index(&Filter::default(), &index_path).await?;

    let index_rec = read_yaml_file::<IndexRec>(&index_path).map_err(other_error)?;

    let mut versions = Vec::new();
    for version_rec in index_rec.versions {
        versions.push(version_rec.version.parse::<GoVersion>()?);
    }

    versions.sort();

    for version in versions {
        println!("{}", version.raw);
    }

    Ok(())
}
