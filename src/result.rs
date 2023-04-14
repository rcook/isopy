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
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    User { message: String },
    Reportable { message: String, info: ErrorInfo },
    Fatal { message: String },
    Other { inner: Box<dyn std::error::Error> },
}

#[derive(Debug)]
pub enum ErrorInfo {
    CouldNotInferIsopyDir,
}

impl<E> From<E> for Error
where
    E: std::error::Error + 'static,
{
    fn from(e: E) -> Self {
        other(Box::new(e))
    }
}

pub fn other(inner: Box<dyn std::error::Error>) -> Error {
    Error::Other { inner }
}

pub fn user<M>(message: M) -> Error
where
    M: Into<String>,
{
    Error::User {
        message: message.into(),
    }
}

pub fn could_not_infer_isopy_dir() -> Error {
    Error::Reportable {
        message: String::from(
            "Could not infer isopy cache directory location: please specify using --dir option",
        ),
        info: ErrorInfo::CouldNotInferIsopyDir,
    }
}

pub fn fatal<M>(message: M) -> Error
where
    M: Into<String>,
{
    Error::Fatal {
        message: message.into(),
    }
}
