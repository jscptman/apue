use std::{io, os::fd::AsRawFd};

use nix::unistd;

fn main() {
    let stdout = io::stdout().lock();
    let stdout_fileno = stdout.as_raw_fd();
    let fd1 = unistd::dup(stdout_fileno).unwrap();
    let fd2 = unistd::dup(stdout_fileno).unwrap();
    let fd3 = unistd::dup(stdout_fileno).unwrap();
    println!("🚀 fd: {fd1}");
    println!("🚀 fd: {fd2}");
    println!("🚀 fd: {fd3}");
}
