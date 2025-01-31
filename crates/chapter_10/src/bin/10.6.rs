use std::{
    fs::{File, OpenOptions},
    io::{Read, Result as IOResult, Seek, SeekFrom, Write},
    sync::Mutex,
};

use nix::{
    sys::signal::{
        self, SaFlags, SigAction, SigHandler, SigSet,
        SigmaskHow::SIG_SETMASK,
        Signal::{SIGKILL, SIGUSR1, SIGUSR2},
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
const MAX_WRITE_COUNT: u32 = 30;
fn main() {
    // 阻塞所有信号
    signal::sigprocmask(SIG_SETMASK, Some(&SigSet::all()), None).unwrap_or_else(|errno| {
        eprintln!("Failed to block signals: {:?}", errno.desc());
        std::process::exit(1);
    });
    // 创建文件
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open("temp/10.6.txt")
        .unwrap_or_else(|error| {
            eprintln!("Failed to open file: {:?}", error.kind());
            std::process::exit(error.raw_os_error().unwrap());
        });
    file.write_all(b"0").unwrap_or_else(|error| {
        eprintln!("Failed to write file: {:?}", error.kind());
        std::process::exit(error.raw_os_error().unwrap());
    });
    COUNTER_FILE.lock().unwrap().replace(file);
    match unsafe { unistd::fork() } {
        Ok(Parent { child }) => {
            CHILD_ID.lock().unwrap().replace(child);
            PARENT_ID.lock().unwrap().replace(unistd::getpid());
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
                    std::process::exit(1);
                });
            };
            let mut sigset = SigSet::all();
            sigset.remove(SIGUSR1);
            sigset.suspend().unwrap_or_else(|errno| {
                eprintln!("Parent failed to suspend: {:?}", errno.desc());
                std::process::exit(1);
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
                std::process::exit(1);
            });
        }
        Err(errno) => {
            eprintln!("Failed to fork: {:?}", errno.desc());
        }
    }
}

extern "C" fn sig_handler(signo: i32) {
    block_all_signals();
    let which_process = if signo == SIGUSR1 as i32 {
        "parent"
    } else {
        "child"
    };

    {
        let file = &mut *COUNTER_FILE.lock().unwrap();
        let current_count = increase_counter(file.as_mut().unwrap()).unwrap_or_else(|error| {
            eprintln!("failed to increase counter: {:?}", error.kind());
            std::process::exit(error.raw_os_error().unwrap());
        });
        println!(
            "🚀 {} process added one, current_count={}",
            which_process, current_count
        );
        if current_count == MAX_WRITE_COUNT {
            kill_process(PARENT_ID.lock().unwrap().unwrap());
            return;
        }
    } // 新增完成后解锁

    if which_process == "parent" {
        signal::kill(CHILD_ID.lock().unwrap().unwrap(), SIGUSR2).unwrap_or_else(|errno| {
            eprintln!(
                "Failed to send SIGUSR2 signal to process: {:?}",
                errno.desc()
            );
            std::process::exit(1);
        });
        let mut sig_set = SigSet::all();
        sig_set.remove(SIGUSR1);
        sig_set.suspend().unwrap_or_else(|errno| {
            eprintln!("Parent failed to suspend: {:?}", errno.desc());
            std::process::exit(1);
        });
    } else if which_process == "child" {
        signal::kill(PARENT_ID.lock().unwrap().unwrap(), SIGUSR1).unwrap_or_else(|errno| {
            eprintln!(
                "Failed to send SIGUSR1 signal to process: {:?}",
                errno.desc()
            );
            std::process::exit(1);
        });
        let mut sig_set = SigSet::all();
        sig_set.remove(SIGUSR2);
        sig_set.suspend().unwrap_or_else(|errno| {
            eprintln!("Parent failed to suspend: {:?}", errno.desc());
            std::process::exit(1);
        });
    }
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
        std::process::exit(1);
    });
}

fn block_all_signals() {
    signal::sigprocmask(SIG_SETMASK, Some(&SigSet::all()), None).unwrap_or_else(|errno| {
        eprintln!("Failed to block all signals: {:?}", errno.desc());
        std::process::exit(1);
    });
}
