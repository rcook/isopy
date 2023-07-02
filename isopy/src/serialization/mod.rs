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
pub mod adoptium;

mod asset_rec;
mod env_rec;
mod index_rec;
mod openjdk_env_rec;
mod openjdk_version_rec;
mod package_rec;
mod python_env_rec;
mod python_version_rec;
mod repositories_rec;
mod repository_rec;

pub use self::asset_rec::AssetRec;
pub use self::env_rec::EnvRec;
pub use self::index_rec::IndexRec;
pub use self::openjdk_env_rec::OpenJdkEnvRec;
pub use self::openjdk_version_rec::OpenJdkVersionRec;
pub use self::package_rec::PackageRec;
pub use self::python_env_rec::PythonEnvRec;
pub use self::python_version_rec::PythonVersionRec;
pub use self::repositories_rec::RepositoriesRec;
pub use self::repository_rec::RepositoryRec;
