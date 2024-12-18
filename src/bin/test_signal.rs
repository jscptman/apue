use std::{
    ffi::OsStr,
    io,
    os::unix::process::{CommandExt, ExitStatusExt},
    process::{self, Command, ExitStatus},
};

fn main() -> Result<(), io::Error> {
    let mut cmd = Command::new("program");
    let mut status = unsafe {
        cmd.pre_exec(|| {
            process::exit(8);
        })
        .status()
        .unwrap()
    };
    pr_exit(cmd.get_program(), &status);

    cmd = Command::new("program2");
    status = unsafe { cmd.pre_exec(|| process::abort()).status().unwrap() };
    pr_exit(cmd.get_program(), &status);

    cmd = Command::new("program3");
    status = unsafe {
        #[allow(unconditional_panic)]
        cmd.pre_exec(|| {
            let _ = 1.0 / 0.0;
            unreachable!();
        })
        .status()
        .unwrap()
    };
    pr_exit(cmd.get_program(), &status);
    Ok(())
}
fn pr_exit(program: &OsStr, status: &ExitStatus) {
    if let Some(code) = status.code() {
        // by exit
        println!(
            "🚀 normal termination, program = {:?}, exit status = {}",
            program, code
        );
    } else {
        // by signal
        println!(
            "🚀 abnormal termination, program = {:?}, signal number = {}, core file generated = {}",
            program,
            status.signal().unwrap(),
            status.core_dumped()
        );
    }
}
