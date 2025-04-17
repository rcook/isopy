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
use crate::local_package_info::LocalPackageInfo;
use itertools::Itertools;

fn sort_infos(infos: &mut [LocalPackageInfo]) {
    infos.sort_by_cached_key(|i| i.package.metadata.version.clone());
    infos.reverse();
}

// On some platforms, most notably Windows, we will get more than one package
// matching the given tags: these functions will choose the "best" so that
// there is exactly one matching package for a given version and build label

#[cfg(not(target_os = "windows"))]
pub(crate) fn choose_best(mut infos: Vec<LocalPackageInfo>) -> Vec<LocalPackageInfo> {
    sort_infos(&mut infos);

    for (key, group) in &infos
        .iter()
        .chunk_by(|i| i.package.metadata.version.clone())
    {
        assert_eq!(
            1,
            group.count(),
            "More than one viable candidate for package {key}"
        );
    }
    infos
}

// On Windows, we prefer the "shared" library over the "static" library and
// choose the default otherwise
#[cfg(target_os = "windows")]
pub(crate) fn choose_best(mut infos: Vec<LocalPackageInfo>) -> Vec<LocalPackageInfo> {
    sort_infos(&mut infos);

    let mut best_infos = Vec::new();
    for (key, group) in &infos
        .into_iter()
        .chunk_by(|i| i.package.metadata.version.clone())
    {
        let infos = group.collect::<Vec<_>>();
        let info_count = infos.len();
        assert_ne!(0, info_count);

        if info_count > 1 {
            let mut temp_shared = None;
            let mut temp_static = None;
            let mut temp_default = None;

            for info in infos {
                let tags = &info.package.metadata.tags;
                if tags.contains("shared") {
                    assert!(temp_shared.is_none());
                    temp_shared = Some(info);
                } else if tags.contains("static") {
                    assert!(temp_static.is_none());
                    temp_static = Some(info);
                } else {
                    assert!(temp_default.is_none());
                    temp_default = Some(info);
                }
            }

            assert!(
                temp_shared.is_some() || temp_default.is_some() || temp_static.is_some(),
                "No viable candidate for package {key}"
            );

            if let Some(info) = temp_shared {
                best_infos.push(info);
            } else if let Some(info) = temp_default {
                best_infos.push(info);
            } else if let Some(info) = temp_static {
                best_infos.push(info);
            } else {
                unreachable!()
            }
        } else {
            let info = infos
                .into_iter()
                .next()
                .expect("Must have exactly one element");
            best_infos.push(info);
        }
    }

    best_infos
}
