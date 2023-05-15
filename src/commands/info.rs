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
use crate::util::{find_dir_info, print, print_dir_info, print_title};
use anyhow::Result;

pub fn do_info(app: &App) -> Result<()> {
    print_title("Current directory");
    print("Working directory", app.cwd.display());
    if let Some(project) = app.read_project(&app.cwd)? {
        print("Project configuration file", project.config_path.display());
        print("Python version", project.python_version);
        print(
            "Build tag",
            project
                .tag
                .as_ref()
                .map(Tag::to_string)
                .unwrap_or(String::from("(none)")),
        );
    }

    if let Some(dir_info) = find_dir_info(&app.repo, &app.cwd)? {
        print_title("Environment info");
        print_dir_info(&dir_info);
    }

    print_title("Repository information");
    print("Lock file", app.repo.lock_path().display());
    print("Configuration file", app.repo.config_path().display());
    print("Links directory", app.repo.links_dir().display());
    print("Container directory", app.repo.container_dir().display());
    print("Shared directory", app.repo.shared_dir().display());

    Ok(())
}
