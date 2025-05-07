#![cfg(feature = "bin_8_7")]
use nix::{
    dir::Dir,
    fcntl::{self, FcntlArg, FdFlag, OFlag},
    sys::stat::Mode,
};
use std::os::fd::AsFd;
use std::{fs::OpenOptions, io};

fn main() -> Result<(), io::Error> {
    let dir = OpenOptions::new()
        .read(true)
        .open("/")
        .expect("path open failed");
    let mut close_exec_flag = fcntl::fcntl(dir.as_fd(), FcntlArg::F_GETFD).expect("fcntl failed");
    println!(
        "🚀 rust std open close_exec_flag: {}, global FD_CLOSE_EXEC value is: {:?}",
        close_exec_flag,
        FdFlag::FD_CLOEXEC.bits()
    );

    let dir = Dir::open("/", OFlag::O_RDONLY, Mode::empty())?;
    close_exec_flag = fcntl::fcntl(dir.as_fd(), FcntlArg::F_GETFD).expect("fcntl failed");
    println!(
        "🚀 nix Dir::open close_exec_flag: {}, global FD_CLOSE_EXEC value is: {:?}",
        close_exec_flag,
        FdFlag::FD_CLOEXEC.bits()
    );

    let dir = fcntl::open("/", OFlag::O_RDONLY, Mode::empty())?;
    close_exec_flag = fcntl::fcntl(&dir, FcntlArg::F_GETFD).expect("fcntl failed");
    println!(
        "🚀 nix fcntl::open close_exec_flag: {}, global FD_CLOSE_EXEC value is: {:?}",
        close_exec_flag,
        FdFlag::FD_CLOEXEC.bits()
    );

    fcntl::fcntl(&dir, FcntlArg::F_SETFD(FdFlag::all()))?;
    close_exec_flag = fcntl::fcntl(dir, FcntlArg::F_GETFD).expect("fcntl failed");
    println!(
        "🚀 after fcntl::setfd close_exec_flag: {}, global FD_CLOSE_EXEC value is: {:?}",
        close_exec_flag,
        FdFlag::FD_CLOEXEC.bits()
    );
    Ok(())
}
