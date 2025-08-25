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
use crate::op::{Op, OpProgress};
use crate::result::Result;
use crate::ui::Ui;
use std::sync::{LazyLock, RwLock};

static GLOBAL: LazyLock<RwLock<Option<Ui>>> = LazyLock::new(|| RwLock::new(None));

#[allow(unused)]
pub fn init_ui(enable_logger: bool) -> Result<()> {
    let mut writer = GLOBAL.write().expect("lock is poisoned");
    assert!(
        writer.is_none(),
        "global UI object has already been initialized"
    );
    *writer = Some(Ui::new(enable_logger)?);
    drop(writer);
    Ok(())
}

#[allow(unused)]
pub fn begin_operation(len: Option<OpProgress>) -> Result<Op> {
    let reader = GLOBAL.read().expect("lock is poisoned");
    reader
        .as_ref()
        .expect("global UI object not initialized")
        .begin_operation(len)
}
