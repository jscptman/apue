use std::{
    fs::{File, OpenOptions},
    io::{Read, Result as IOResult, Seek, Write},
    process,
    sync::Mutex,
};

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
static PID: Mutex<Option<Pid>> = Mutex::new(None);
static PPID: Mutex<Option<Pid>> = Mutex::new(None);
fn main() {
    let mut sigset_old = SigSet::empty();
    // 阻塞所有信号
    signal::sigprocmask(SIG_SETMASK, Some(&SigSet::all()), Some(&mut sigset_old)).unwrap_or_else(
        |errno| {
            eprintln!("Failed to block signals: {:?}", errno.desc());
            std::process::exit(1);
        },
    );
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
            PID.lock().unwrap().replace(unistd::getpid());
            PPID.lock().unwrap().replace(child);
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
    println!(
        "🚀 received signo: {}",
        Signal::try_from(signo).unwrap_or_else(|errno| {
            eprintln!("Failed to convert signal: {:?}", errno.desc());
            std::process::exit(1);
        })
    );
    let which_process = if signo == SIGUSR1 as i32 {
        "parent"
    } else {
        "child"
    };
    let file = &mut *COUNTER_FILE.lock().unwrap();
    let desc = Signal::try_from(signo).unwrap().as_str().to_string();
    let current_count = increase_counter(file.as_mut().unwrap(), desc).unwrap_or_else(|error| {
        eprintln!("failed to increase counter: {:?}", error.kind());
        std::process::exit(error.raw_os_error().unwrap());
    });
    if current_count == 10 {
        if which_process == "parent" {
            signal::kill(PID.lock().unwrap().unwrap(), SIGKILL).unwrap_or_else(|errno| {
                eprintln!("Failed to send SIGIOT signal to child: {:?}", errno.desc());
                std::process::exit(1);
            });
            process::exit(0);
        } else {
            signal::kill(PPID.lock().unwrap().unwrap(), SIGKILL).unwrap_or_else(|errno| {
                eprintln!("Failed to send SIGIOT signal to parent: {:?}", errno.desc());
                std::process::exit(1);
            });
            process::exit(0);
        }
    }
}

fn increase_counter(file: &mut File, desc: impl AsRef<str>) -> IOResult<u32> {
    let mut buf = String::new();
    file.seek(std::io::SeekFrom::Start(0))?;
    let length = file.read_to_string(&mut buf)?;
    println!("🚀 buf: {}", buf);
    let counter = buf[..length].parse::<u32>().unwrap();
    let write_content = format!("{}, counter={}", desc.as_ref(), counter + 1);
    file.write_all(write_content.as_bytes())?;
    Ok(counter + 1)
}
