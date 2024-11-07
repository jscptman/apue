use libc::PATH_MAX;
use std::{
    env::{self, set_current_dir},
    fs,
};

fn main() {
    let leaf_dir_name = "ABCDEFGHIJ";
    set_current_dir("/home/jscptman").unwrap();
    loop {
        match fs::create_dir(&leaf_dir_name) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("function create_dir occurs an error: {:?}\nleaf_directory_path: {}\nleaf_path_length: {}", e, leaf_dir_name, leaf_dir_name.len());
                return;
            }
        }
        set_current_dir(&leaf_dir_name).unwrap_or_else(|e| {
            panic!(
                "function set_current_dir occurs an error: {:?}, leaf_dir_name is: {}",
                e, leaf_dir_name
            )
        });
        let current_dir = env::current_dir()
            .unwrap_or_else(|e| panic!("function current_dir occurs an error: {:?}", e));
        let current_dir = current_dir.as_os_str();
        if current_dir.len() > PATH_MAX as usize {
            println!(
                "🚀 current_dir: {:?}, length: {}",
                current_dir,
                current_dir.len()
            );
            break;
        }
    }
}
