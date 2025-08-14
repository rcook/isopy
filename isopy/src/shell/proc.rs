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
use anyhow::{Result, anyhow, bail};
use sysinfo::{Pid, Process, ProcessesToUpdate, System, get_current_pid};

pub(crate) fn get_pid() -> Result<Pid> {
    get_current_pid().or(Err(anyhow!("Failed to get process ID")))
}

pub(crate) fn get_process_from_pid(system: &mut System, pid: Pid) -> Result<&Process> {
    if system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true) == 1 {
        system
            .process(pid)
            .ok_or_else(|| anyhow!("Failed to get process info"))
    } else {
        bail!("Failed to refresh process")
    }
}

pub(crate) fn get_parent_pid(process: &Process) -> Result<Pid> {
    process
        .parent()
        .ok_or_else(|| anyhow!("failed to get parent PID"))
}
