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
#[derive(Debug)]
pub(crate) enum Status {
    Success,
    UserError(String),
}

#[macro_export]
macro_rules! success_quiet {
    () => {
        return ::std::result::Result::Ok($crate::status::Status::Success);
    };
}
pub(crate) use success_quiet;

#[macro_export]
macro_rules! success {
    () => {{
        ::log::info!("isopy completed successfully");
        return ::std::result::Result::Ok($crate::status::Status::Success);
    }};

    ($($arg: tt)*) => {{
        ::log::info!($($arg)*);
        return ::std::result::Result::Ok($crate::status::Status::Success);
    }};
}
pub(crate) use success;

macro_rules! user_error {
    ($($arg: tt)*) => {{
        return ::std::result::Result::Ok($crate::status::Status::UserError(std::format!($($arg)*)));
    }};
}
pub(crate) use user_error;
