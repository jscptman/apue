use std::{
    fs::{File, OpenOptions},
    io::{Read, Result as IOResult, Seek, SeekFrom, Write},
    process,
    sync::Mutex,
};

use nix::libc::c_int;
use nix::{
    sys::signal::{
        self, SaFlags, SigAction, SigHandler, SigSet,
        SigmaskHow::SIG_SETMASK,
        Signal::{self, SIGKILL, SIGUSR1, SIGUSR2},
    },
    unistd::{
        self,
        ForkResult::{Child, Parent},
        Pid,
    },
};

static COUNTER_FILE: Mutex<Option<File>> = Mutex::new(None);
static CHILD_ID: Mutex<Option<Pid>> = Mutex::new(None); // child process id
static PARENT_ID: Mutex<Option<Pid>> = Mutex::new(None); // parent process id
const MAX_WRITE_COUNT: u32 = 300;
fn main() {
    // 阻塞所有信号
    block_all_signals();
    // 初始化文件
    COUNTER_FILE.lock().unwrap().replace(init_file());
    match unsafe { unistd::fork() } {
        Ok(Parent { child }) => {
            CHILD_ID.lock().unwrap().replace(child);
            println!("🚀 parentID: {}, childId: {}", unistd::getpid(), child);
            unsafe {
                signal::sigaction(
                    SIGUSR1,
                    &SigAction::new(
                        SigHandler::Handler(sig_handler),
                        SaFlags::empty(),
                        SigSet::all(),
                    ),
                )
                .unwrap_or_else(|errno| {
                    eprintln!("Failed to set SIGUSR1 signal handler: {:?}", errno.desc());
                    process::exit(1);
                });
            };
            let mut sigset = SigSet::all();
            sigset.remove(SIGUSR1);
            sigset.suspend().unwrap_or_else(|errno| {
                eprintln!("Parent failed to suspend: {:?}", errno.desc());
                process::exit(1);
            });
        }
        Ok(Child) => {
            PARENT_ID.lock().unwrap().replace(unistd::getppid());
            unsafe {
                signal::sigaction(
                    SIGUSR2,
                    &SigAction::new(
                        SigHandler::Handler(sig_handler),
                        SaFlags::empty(),
                        SigSet::all(),
                    ),
                )
                .unwrap();
            };
            let mut sigset = SigSet::all();
            sigset.remove(SIGUSR2);
            sigset.suspend().unwrap_or_else(|errno| {
                eprintln!("Child failed to suspend: {:?}", errno.desc());
                process::exit(1);
            });
        }
        Err(errno) => {
            eprintln!("Failed to fork: {:?}", errno.desc());
        }
    }
}

extern "C" fn sig_handler(signo: c_int) {
    let signal = Signal::try_from(signo).unwrap();
    let current_count = increase_counter(COUNTER_FILE.lock().unwrap().as_mut().unwrap())
        .unwrap_or_else(|error| {
            eprintln!("failed to increase counter: {:?}", error.kind());
            process::exit(error.raw_os_error().unwrap());
        });
    println!(
        "🚀 process={}, current_count={}",
        process::id(),
        current_count
    );
    let reach_increase_upper_limit = current_count == MAX_WRITE_COUNT;
    handle_sigusr(reach_increase_upper_limit, signal);
}

fn increase_counter(file: &mut File) -> IOResult<u32> {
    let mut buf = String::new();
    file.seek(SeekFrom::Start(0))?;
    let length = file.read_to_string(&mut buf)?;
    file.set_len(0)?;
    let counter = buf[..length].parse::<u32>().unwrap();
    let write_content = format!("{}", counter + 1);
    file.seek(SeekFrom::Start(0))?;
    file.write_all(write_content.as_bytes())?;
    Ok(counter + 1)
}

fn kill_process(pid: Pid) {
    signal::kill(pid, SIGKILL).unwrap_or_else(|errno| {
        eprintln!(
            "Failed to send SIGKILL signal to process: {:?}",
            errno.desc()
        );
        process::exit(1);
    });
}

fn block_all_signals() {
    signal::sigprocmask(SIG_SETMASK, Some(&SigSet::all()), None).unwrap_or_else(|errno| {
        eprintln!("Failed to block all signals: {:?}", errno.desc());
        process::exit(1);
    });
}

fn init_file() -> File {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open("10.6.txt")
        .unwrap_or_else(|error| {
            eprintln!("Failed to open file: {:?}", error.kind());
            process::exit(error.raw_os_error().unwrap());
        });
    file.write_all(b"0").unwrap_or_else(|error| {
        eprintln!("Failed to write file: {:?}", error.kind());
        process::exit(error.raw_os_error().unwrap());
    });
    file
}

fn handle_sigusr(reach_increase_upper_limit: bool, current_signal: Signal) {
    let (target_process, target_signal) = if current_signal == SIGUSR1 {
        (CHILD_ID.lock().unwrap().unwrap(), SIGUSR2)
    } else if current_signal == SIGUSR2 {
        (PARENT_ID.lock().unwrap().unwrap(), SIGUSR1)
    } else {
        panic!("got unexpected signal: {:?}", current_signal);
    };
    if reach_increase_upper_limit {
        kill_process(target_process);
        process::exit(0);
    }
    block_all_signals();
    signal::kill(target_process, target_signal).unwrap_or_else(|errno| {
        eprintln!(
            "Failed to send SIGUSR2 signal to process: {:?}",
            errno.desc()
        );
        process::exit(1);
    });
    let mut sig_set = SigSet::all();
    sig_set.remove(current_signal);
    sig_set.suspend().unwrap_or_else(|errno| {
        eprintln!("Parent failed to suspend: {:?}", errno.desc());
        process::exit(1);
    });
}
