use nix::libc::{c_int, siginfo_t};
use nix::sys::resource::{self, Resource::RLIMIT_FSIZE, RLIM_INFINITY};
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, SIGXFSZ};
use std::ffi::c_void;
use std::io::{Read, Result as IOResult, Write};
use std::{io, process};

const BUFFSIZE: usize = 100;

fn main() -> IOResult<()> {
    resource::setrlimit(RLIMIT_FSIZE, 1024, RLIM_INFINITY)?;
    let mut mask = SigSet::all();
    mask.remove(SIGXFSZ);
    unsafe {
        signal::sigaction(
            SIGXFSZ,
            &SigAction::new(
                SigHandler::SigAction(sigxfsz_handler),
                SaFlags::SA_SIGINFO,
                mask,
            ),
        )
        .unwrap_or_else(|errno| panic!("sigaction failed with {}", errno));
    }
    let mut buf = [0_u8; BUFFSIZE];
    let read_bytes = io::stdin().read(&mut buf)?;
    let writen_bytes = io::stdout()
        .write(&buf[..read_bytes])
        .unwrap_or_else(|error| {
            eprintln!("write occurs an error: {}", error);
            process::exit(-1)
        });
    if writen_bytes < read_bytes {
        println!("partial write n={}", writen_bytes)
    }
    Ok(())
}
extern "C" fn sigxfsz_handler(signo: c_int, siginfo: *mut siginfo_t, context: *mut c_void) {
    println!("handler called with {:?}", signo);
}
