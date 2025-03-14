#![cfg(feature = "signal")]
use core::panic;
use nix::{Result, errno::Errno, sys::signal::Signal};
use std::io::stdin;
use std::process;
use std::str::{self, FromStr};
fn main() {
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap_or_else(|error| {
            eprintln!("read_line failed");
            panic!("error: {:?}", error.kind());
        });
        if buf == "\n" {
            continue;
        }
        let input = i32::from_str(buf.trim()).unwrap_or_else(|_| {
            eprintln!("Invalid input, except number found {}", buf);
            process::exit(0);
        });
        match sig2str(input) {
            Ok(signal) => println!("signo: {}, signal: {}", input, signal),
            Err(e) => eprintln!("errno: {}, {}", Errno::last_raw(), e.desc()),
        }
    }
}

fn sig2str(signo: i32) -> Result<&'static str> {
    let signal = Signal::try_from(signo)?;
    Ok(signal.as_str())
}
