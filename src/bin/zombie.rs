use std::{process::Command, thread, time::Duration};
fn main() {
    let mut cmd = Command::new("echo");
    cmd.arg("echo finish");
    cmd.spawn().expect("echo occurs an error");
    thread::sleep(Duration::new(3, 0));
    let mut cmd = Command::new("ps");
    cmd.args(["-o", "pid,ppid,state,tty,command"]);
    cmd.status().expect("ps occurs an error");
}
