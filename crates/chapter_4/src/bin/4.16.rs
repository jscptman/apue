use nix::unistd::{self, PathconfVar};
use std::{
    env::{self, set_current_dir},
    fs,
};

fn main() {
    let mut total = 0;
    const CURRENT_DIR: &str = "/home/jscptman";
    const DEFAULT_PATH_MAX: i64 = 4096;
    let path_max = unistd::pathconf(CURRENT_DIR, PathconfVar::PATH_MAX)
        .expect("get PATH_MAX occurs an error")
        .unwrap_or(DEFAULT_PATH_MAX);
    let file_name_max = unistd::pathconf(CURRENT_DIR, PathconfVar::NAME_MAX)
        .expect("get NAME_MAX occurs an error")
        .unwrap();
    let leaf_dir = &"a".repeat(file_name_max as usize);
    println!("🚀 path_max={}", path_max);
    println!("🚀 name_max={}", file_name_max);
    set_current_dir(CURRENT_DIR).unwrap();
    loop {
        match fs::create_dir(leaf_dir) {
            Ok(_) => {
                total += 1;
            }
            Err(e) => {
                eprintln!(
                    "function create_dir occurs an error: {:?}\nleaf_directory_path: {}\nleaf_path_length: {}",
                    e,
                    leaf_dir,
                    leaf_dir.len()
                );
                return;
            }
        }
        set_current_dir(leaf_dir).unwrap_or_else(|e| {
            panic!(
                "function set_current_dir occurs an error: {}, leaf_dir_name is: {}",
                e, leaf_dir
            )
        });
        let current_dir = env::current_dir()
            .unwrap_or_else(|e| panic!("function current_dir occurs an error: {:?}", e));
        let current_dir = current_dir.as_os_str();

        if current_dir.len() >= 3 * path_max as usize {
            println!("🚀 total={}", total);
            println!("🚀 current_dir: {:?}", current_dir);
            println!("🚀 current_dir_length={}", current_dir.len());
            break;
        }
    }
}
