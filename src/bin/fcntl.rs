use std::{fs::OpenOptions, os::fd::AsRawFd, path::Path};

use libc::{F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_GETFL};

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(Path::new("docs/test.txt"))
        .unwrap();
    let fd: i32 = file.as_raw_fd();
    unsafe {
        println!("🚀 fd: {fd}");
        println!("🚀 fd_flag: {}", libc::fcntl(fd, F_GETFD));
        println!("🚀 fd_status: {:b}", libc::fcntl(fd, F_GETFL));
        // 复制fd -> fd1
        let fd1 = libc::fcntl(fd, F_DUPFD);
        println!("🚀 fd1: {fd1}");
        println!("🚀 fd1_flag: {}", libc::fcntl(fd1, F_GETFD));
        println!("🚀 fd1_status: {:b}", libc::fcntl(fd1, F_GETFL));

        // 复制fd -> fd2
        let fd2 = libc::fcntl(fd, F_DUPFD_CLOEXEC);
        println!("🚀 fd2: {fd2}");
        println!("🚀 fd2_flag: {}", libc::fcntl(fd2, F_GETFD));
        println!("🚀 fd2_status: {:b}", libc::fcntl(fd2, F_GETFL));
    };
}
