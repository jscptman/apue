use core::panic;
use std::cmp;

use nix::{
    errno::Errno,
    sys::resource::{self, Resource},
    unistd::{self, SysconfVar},
};

fn main() {
    println!("🚀 system OPEN_MAX={}", open_max());
}

fn open_max() -> i64 {
    const OPEN_MAX_GUESS: i64 = 255;
    let limit = unistd::sysconf(SysconfVar::OPEN_MAX).unwrap_or_else(|errno| {
        panic!(
            "{} sysconf occurs an error, errno={}, {}",
            line!(),
            Errno::last_raw(),
            errno.desc()
        );
    });
    match limit {
        Some(limit) => {
            return limit;
        }
        None => {
            let (software_limit, hardware_limit) = resource::getrlimit(Resource::RLIMIT_NOFILE)
                .unwrap_or_else(|errno| {
                    panic!(
                        "{} getrlimit occurs an error, errno={}, {}",
                        line!(),
                        Errno::last_raw(),
                        errno.desc()
                    );
                });
            println!(
                "🚀 software_limit={}, hardware_limit={}",
                software_limit, hardware_limit
            );
            if software_limit == resource::RLIM_INFINITY
                && hardware_limit == resource::RLIM_INFINITY
            {
                println!("🚀 use guess");
                return OPEN_MAX_GUESS;
            } else {
                return cmp::min(software_limit, hardware_limit) as i64;
            }
        }
    }
}
