use std::{io, process};

use nix::{
    errno::Errno,
    sys::wait,
    unistd::{self, ForkResult, Pid},
};

fn main() {
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => {
            let pid = Pid::this();
            print_process_ids(
                "parent",
                pid,
                unistd::getppid(),
                unistd::getpgrp(),
                unistd::tcgetpgrp(io::stdout()).unwrap_or_else(|e| {
                    print_sys_error(line!(), e);
                    process::exit(0)
                }),
                unistd::getsid(Some(pid)).unwrap_or_else(|e| {
                    print_sys_error(line!(), e);
                    process::exit(0)
                }),
            );
            wait::wait().unwrap_or_else(|errno| {
                print_sys_error(line!(), errno);
                process::exit(0);
            });
        }
        Ok(ForkResult::Child) => {
            let pid = Pid::this();
            print_process_ids(
                "child before setsid",
                pid,
                unistd::getppid(),
                unistd::getpgrp(),
                unistd::tcgetpgrp(io::stdout()).unwrap_or_else(|e| {
                    print_sys_error(line!(), e);
                    process::exit(0)
                }),
                unistd::getsid(Some(pid)).unwrap_or_else(|e| {
                    print_sys_error(line!(), e);
                    process::exit(0)
                }),
            );
            unistd::setsid().unwrap_or_else(|e| {
                print_sys_error(line!(), e);
                process::exit(0)
            });
            let pid = Pid::this();
            print_process_ids(
                "child after setsid",
                pid,
                unistd::getppid(),
                unistd::getpgrp(),
                unistd::tcgetpgrp(io::stdout()).unwrap_or_else(|e| {
                    print_sys_error(line!(), e);
                    Pid::from_raw(-1)
                }),
                unistd::getsid(Some(pid)).unwrap_or_else(|e| {
                    print_sys_error(line!(), e);
                    process::exit(0)
                }),
            );
        }
        Err(e) => {
            print_sys_error(line!(), e);
        }
    }
}

fn print_process_ids(label: &str, pid: Pid, ppid: Pid, pgrp: Pid, tcpgrp: Pid, sid: Pid) {
    println!(
        "🚀 {}: process_id: {}, parent_process_id: {}, process_group_id: {}, foreground_process_group_id: {}, session_id: {}",
        label, pid, ppid, pgrp, tcpgrp, sid
    );
}

fn print_sys_error(line_no: u32, errno: Errno) {
    println!(
        "🚀 line={} occurs an error, errno={}, {}",
        line_no,
        Errno::last_raw(),
        errno.desc()
    );
}
