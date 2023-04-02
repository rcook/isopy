use crate::result::{fatal, Result};
use sysinfo::{get_current_pid, Pid, Process, ProcessExt, System, SystemExt};

pub fn get_pid() -> Result<Pid> {
    get_current_pid().or(Err(fatal("Failed to get process ID")))
}

pub fn get_process_from_pid<'a>(system: &'a mut System, pid: Pid) -> Result<&'a Process> {
    if system.refresh_process(pid) {
        system
            .process(pid)
            .ok_or(fatal("Failed to get process info"))
    } else {
        Err(fatal("Failed to refresh process"))
    }
}

pub fn get_parent_pid(process: &Process) -> Result<Pid> {
    process.parent().ok_or(fatal("Failed to get parent PID"))
}
