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
use crate::app::App;
use crate::serialization::ProjectEnvironmentRecord;
use crate::util::{print_env, print_link, print_manifest, print_title};
use anyhow::Result;
use joatmon::read_yaml_file;

pub async fn do_list(app: &App) -> Result<()> {
    let manifests = app.repo.list_manifests()?;
    if !manifests.is_empty() {
        print_title("Manifests");
        for (idx, manifest) in manifests.iter().enumerate() {
            println!("  ({}) {}", idx + 1, manifest.meta_id());
            print_manifest(manifest);

            let env_yaml_path = manifest.data_dir().join("env.yaml");
            if env_yaml_path.is_file() {
                let rec = read_yaml_file::<ProjectEnvironmentRecord, _>(env_yaml_path)?;
                print_env(&rec);
            }
        }
    }

    let links = app.repo.list_links()?;
    if !links.is_empty() {
        print_title("Links");
        for (idx, link) in links.iter().enumerate() {
            println!("  ({}) {} -> {}", idx + 1, link.link_id(), link.meta_id());
            print_link(link);
        }
    }

    Ok(())
}
