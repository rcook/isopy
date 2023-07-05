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
use crate::descriptor_info::DescriptorInfo;
use crate::print::print;
use crate::status::Status;
use anyhow::Result;
use colored::Colorize;
use std::sync::Arc;

pub async fn do_available(app: &App) -> Result<Status> {
    for plugin in &app.registry.plugins {
        let package_infos = plugin
            .product
            .get_package_infos(app.repo.shared_dir())
            .await?;
        if !package_infos.is_empty() {
            print(&format!(
                "{} ({})",
                plugin.product.name().cyan(),
                plugin.product.url().as_str().bright_magenta()
            ));

            for package_info in package_infos {
                let descriptor_info = DescriptorInfo {
                    plugin: Arc::clone(plugin),
                    descriptor: package_info.descriptor,
                };
                print(&format!(
                    "  {:<30} {}",
                    format!("{descriptor_info}").bright_yellow(),
                    package_info.file_name.display()
                ));
            }
        }
    }
    Ok(Status::OK)
}
