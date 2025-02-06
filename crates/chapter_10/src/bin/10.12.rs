use std::{
    cmp,
    fs::File,
    io::{BufWriter, Result as IOResult, Write},
};

use nix::{
    sys::signal::{self, SaFlags, SigAction, SigHandler, SIGALRM},
    unistd::alarm,
};
const MAX_RW_COUNT_HALF: usize = 2147479552 / 2; // 内核最大支持的单次写入字节数的一半
const BUFFSIZE: usize = MAX_RW_COUNT_HALF; // 设置缓冲区大小
fn main() -> IOResult<()> {
    unsafe {
        signal::sigaction(
            SIGALRM,
            &SigAction::new(
                SigHandler::Handler(sigalrm_handler),
                SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap_or_else(|errno| panic!("sigaction failed with {}", errno));
    };
    let mut stream = BufWriter::with_capacity(BUFFSIZE, File::create("10_12")?);
    println!("🚀 capacity={}", stream.capacity());
    let tmp = vec![b'A'; 5_000_000_000]; // 5GB的数据
    println!("🚀 begin write");
    alarm::set(2); // 设置2秒后发送SIGALRM信号
    let mut last_write = 0;
    let mut write_bound = cmp::min(tmp.len(), last_write + BUFFSIZE);
    while let Ok(n) = stream.write(&tmp[last_write..write_bound]) {
        if n == 0 {
            break;
        }
        println!("🚀 written={}", n);
        last_write += n;
        write_bound = cmp::min(tmp.len(), last_write + BUFFSIZE);
    }
    println!("🚀 total written={}", last_write);
    Ok(())
}
extern "C" fn sigalrm_handler(signo: i32) {
    println!("🚀 received signo={}", signo);
}
