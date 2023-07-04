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
use crate::registry::ProductInfo;
use crate::{app::App, status::Status};
use anyhow::Result;
use log::error;
use std::path::PathBuf;
use std::rc::Rc;

pub async fn do_init_config(app: &App) -> Result<Status> {
    if app.repo.get(&app.cwd)?.is_some() {
        error!("directory {} already has an environment", app.cwd.display());
        return Ok(Status::Fail);
    }

    let Some((product_info, project_config_path)) = get_product_info_and_project_config_path(app) else {
        error!("no project configuration file was found in {}", app.cwd.display());
        return Ok(Status::Fail)
    };

    let descriptor = product_info
        .product
        .read_project_config_file(&project_config_path)?;

    app.init_project(&product_info, descriptor.as_ref()).await?;

    Ok(Status::OK)
}

fn get_product_info_and_project_config_path(app: &App) -> Option<(Rc<ProductInfo>, PathBuf)> {
    for product_info in &app.registry.product_infos {
        let project_config_path = app
            .cwd
            .join(product_info.product.project_config_file_name());
        if project_config_path.is_file() {
            return Some((Rc::clone(product_info), project_config_path));
        }
    }

    None
}
