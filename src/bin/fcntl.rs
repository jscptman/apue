use std::{fs::OpenOptions, os::fd::AsRawFd, path::Path};

use nix::fcntl::{self, FcntlArg};

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .truncate(false)
        .read(true)
        .create(true)
        .open(Path::new("docs/test.txt"))
        .unwrap();
    let fd = file.as_raw_fd();
    println!("🚀 fd: {fd}");
    println!(
        "🚀 fd_flag: {}",
        fcntl::fcntl(fd, FcntlArg::F_GETFD).unwrap()
    );
    println!(
        "🚀 fd_status: {:b}",
        fcntl::fcntl(fd, FcntlArg::F_GETFL).unwrap()
    );
    // 复制fd -> fd1
    let fd1 = fcntl::fcntl(fd, FcntlArg::F_DUPFD(fd)).unwrap();
    println!("🚀 fd1: {fd1}");
    println!(
        "🚀 fd1_flag: {}",
        fcntl::fcntl(fd1, FcntlArg::F_GETFD).unwrap()
    );
    println!(
        "🚀 fd1_status: {:b}",
        fcntl::fcntl(fd1, FcntlArg::F_GETFL).unwrap()
    );

    // 复制fd -> fd2
    let fd2 = fcntl::fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(fd)).unwrap();
    println!("🚀 fd2: {fd2}");
    println!(
        "🚀 fd2_flag: {}",
        fcntl::fcntl(fd2, FcntlArg::F_GETFD).unwrap()
    );
    println!(
        "🚀 fd2_status: {:b}",
        fcntl::fcntl(fd2, FcntlArg::F_GETFL).unwrap()
    );
}
