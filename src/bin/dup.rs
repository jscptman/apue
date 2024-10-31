use std::io::stdin;

use libc::STDOUT_FILENO;

fn main() {
    println!("🚀 123");
    unsafe {
        let fd1 = libc::dup(STDOUT_FILENO);
        let fd2 = libc::dup(STDOUT_FILENO);
        let fd3 = libc::dup(STDOUT_FILENO);
        println!("🚀 fd: {fd1}");
        println!("🚀 fd: {fd2}");
        println!("🚀 fd: {fd3}");
    };
    stdin().read_line(&mut String::from("")).unwrap();
}
