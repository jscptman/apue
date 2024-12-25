use std::os::fd::RawFd;

use nix::unistd;

fn main() {
    let fd = my_dup2(1, 15);
    println!("result_fd: {}", fd);
}

fn my_dup2(fd1: RawFd, fd2: RawFd) -> RawFd {
    if !(is_valid_fd(fd1) && is_valid_fd(fd2)) {
        return -1;
    }
    let mut fd_temp_vec = Vec::new();
    if fd1 == fd2 {
        fd1
    } else {
        loop {
            let fd_temp = unistd::dup(fd1).expect("dup occurs an error");
            if fd_temp == -1 {
                break -1;
            } else if fd_temp != fd2 {
                fd_temp_vec.push(fd_temp);
            } else {
                for (index, fd) in fd_temp_vec.iter().enumerate() {
                    println!("🚀 fd_temp{}: {}", index, fd);
                }
                break fd_temp;
            }
        }
    }
}

fn is_valid_fd(fd: RawFd) -> bool {
    fd >= 0
}
