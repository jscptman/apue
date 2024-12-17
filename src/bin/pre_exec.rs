use std::{
    io,
    os::{fd::AsRawFd, unix::process::CommandExt},
    process::{self, Command},
};

use libc::close;

fn main() -> Result<(), io::Error> {
    let mut cmd = Command::new("ls");
    let mut child = unsafe {
        cmd.pre_exec(|| {
            let stdout = io::stdout();
            // drop(stdout);
            close(stdout.as_raw_fd());
            process::exit(0)
        });
        cmd.spawn()?
    };
    let status = child.wait()?;
    let stdout = io::stdout();
    drop(stdout);
    // unsafe {
    //         close(stdout.as_raw_fd());
    // }
    println!(
        "🚀 child exit code: {}",
        status.code().expect("child was terminated by a signal")
    );
    Ok(())
}
