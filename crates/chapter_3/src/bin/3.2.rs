#[cfg(feature = "3_2")]
use nix::{errno::Errno, unistd};
use std::os::fd::RawFd;

fn main() {
    let fd = my_dup2(1, 15);
    println!("result_fd: {}", fd);
}

fn my_dup2(fd1: RawFd, fd2: RawFd) -> RawFd {
    let mut fd_temp_vec = Vec::new();
    if fd1 == fd2 {
        fd1
    } else {
        loop {
            let fd_temp = unistd::dup(fd1).unwrap_or_else(|errno| {
                panic!(
                    "{} dup occurs an error, errno={}, {}",
                    line!(),
                    Errno::last_raw(),
                    errno.desc()
                )
            });
            if fd_temp != fd2 {
                fd_temp_vec.push(fd_temp);
            } else {
                for fd in fd_temp_vec.into_iter() {
                    unistd::close(fd).unwrap_or_else(|errno| {
                        panic!(
                            "{} close fd={} occurs an error, errno={}, {}",
                            line!(),
                            fd,
                            Errno::last_raw(),
                            errno.desc()
                        )
                    });
                }
                break fd_temp;
            }
        }
    }
}
