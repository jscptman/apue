use nix::libc::{F_GETFD, fcntl};
#[cfg(feature = "bin_3_2")]
use nix::{errno::Errno, unistd};
use std::ffi::c_int;
use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd, RawFd};
const SOURCE_FD: c_int = 1;
fn main() {
    match unsafe { fcntl(SOURCE_FD, F_GETFD) } {
        -1 => {
            eprintln!("source file is not opened");
        }
        _ => {
            let (source_fd, dst_fd) = unsafe { (OwnedFd::from_raw_fd(SOURCE_FD), 15) };
            let fd = match my_dup2(source_fd.as_fd(), dst_fd) {
                Ok(fd) => fd,
                Err(reason) => {
                    eprintln!("{reason}");
                    return;
                }
            };
            println!("result_fd: {}", fd);
        }
    }
}

fn my_dup2<T: AsFd>(fd1: T, fd2: RawFd) -> Result<RawFd, String> {
    let mut new_fd_vec = Vec::new();
    let raw_fd1 = fd1.as_fd().as_raw_fd();
    if raw_fd1 == fd2 {
        Ok(raw_fd1)
    } else {
        loop {
            let fd_temp = match unistd::dup(fd1.as_fd()) {
                Ok(new_fd) => new_fd,
                Err(errno) => {
                    return Err(format!(
                        "line: {}, dup occurs an error, errno={}, {}",
                        line!(),
                        Errno::last_raw(),
                        errno.desc()
                    ));
                }
            };
            let raw_fd_temp = fd_temp.as_raw_fd();
            if raw_fd_temp != fd2 {
                new_fd_vec.push(fd_temp);
            } else {
                for fd in new_fd_vec {
                    unistd::close(fd).unwrap_or_else(|errno| {
                        panic!(
                            "line: {}, close fd={} occurs an error, errno={}, {}",
                            line!(),
                            raw_fd_temp,
                            Errno::last_raw(),
                            errno.desc()
                        )
                    });
                }
                break Ok(raw_fd_temp);
            }
        }
    }
}
