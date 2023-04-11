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
use crate::object_model::Tag;
use crate::result::Result;

pub fn do_info(app: &App) -> Result<()> {
    println!("Working directory: {}", app.cwd.display());
    println!("isopy directory: {}", app.dir.display());

    if let Some(project) = app.read_project(&app.cwd)? {
        println!(
            "Project configuration file: {}",
            project.config_path.display()
        );
        println!("Python version: {}", project.python_version);
        println!(
            "Build tag: {}",
            project
                .tag
                .as_ref()
                .map(Tag::to_string)
                .unwrap_or(String::from("(none)"))
        );
        let dir = app.project_environment_dir(&project.config_path)?;
        println!("Environment directory: {}", dir.display());
    }
    Ok(())
}
