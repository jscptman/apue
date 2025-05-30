#![allow(clippy::transmute_ptr_to_ref)]
#![cfg(feature = "bin_10_11")]

use nix::libc::{c_int, siginfo_t};
use nix::sys::resource::{self, RLIM_INFINITY, Resource::RLIMIT_FSIZE};
use nix::sys::signal::{self, SIGXFSZ, SaFlags, SigAction, SigHandler, SigSet};
#[cfg(target_os = "linux")]
use nix::ucontext::UContext;
use std::ffi::c_void;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Result as IOResult, Write};
#[cfg(target_os = "linux")]
use std::mem;
const BUFFSIZE: usize = 100;
fn main() -> IOResult<()> {
    let mut mask = SigSet::all();
    mask.remove(SIGXFSZ);
    let tmp = (1..10).map(|i| i.to_string()).collect::<Vec<_>>().join("") + "\n";
    fs::write("temp_infile", tmp.repeat(200))?;
    let mut out_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("temp_outfile")?;
    out_file.set_len(0)?;
    unsafe {
        signal::sigaction(
            SIGXFSZ,
            &SigAction::new(
                SigHandler::SigAction(sigxfsz_handler),
                SaFlags::SA_RESTART | SaFlags::SA_SIGINFO,
                mask,
            ),
        )
        .unwrap_or_else(|errno| panic!("sigaction failed with {}", errno));
    }
    resource::setrlimit(RLIMIT_FSIZE, 1024, RLIM_INFINITY)?;
    let mut in_file = File::open("temp_infile")?;
    let mut buf = [0_u8; BUFFSIZE];
    while let Ok(n) = in_file.read(&mut buf) {
        if n == 0 {
            break;
        }
        if out_file.write(&buf[..n])? != BUFFSIZE {
            println!("partially write");
        }
    }
    Ok(())
}
extern "C" fn sigxfsz_handler(signo: c_int, siginfo: *mut siginfo_t, context: *mut c_void) {
    println!("handler called with {}", signo);
    #[cfg(target_os = "linux")]
    unsafe {
        let ucontext: *mut UContext = mem::transmute(context);
        println!("context: {:?}", *ucontext);
        println!("siginfo: {:?}", *siginfo);
    }
}
