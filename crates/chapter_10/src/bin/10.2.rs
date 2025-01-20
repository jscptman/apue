use core::panic;
use nix::{errno::Errno, sys::signal::Signal, Result};
use std::io::stdin;
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
        let input = i32::from_str(buf.trim()).unwrap_or_else(|error| {
            eprintln!("Invalid input, except number found {}", buf);
            panic!("error: {:?}", error.kind());
        });
        match sig2str(input) {
            Ok(signal) => println!("signo: {}, signal: {}", input, signal),
            Err(e) => eprintln!("errno: {}, 5: {}", Errno::last_raw(), e.desc()),
        }
    }
}

fn sig2str(signo: i32) -> Result<&'static str> {
    let signal = Signal::try_from(signo)?;
    Ok(signal.as_str())
}
