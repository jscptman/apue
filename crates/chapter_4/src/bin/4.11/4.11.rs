#![cfg(feature = "4_11")]
mod summary;
use apue_common::os::unix::fs::file_type_etx::{FILE_TYPES, FileTypeEnum, FileTypeExt};
use std::{env, fs, path::Path};
use summary::FileStatisticSummary;
fn main() {
    let mut args = env::args();
    println!("🚀 {}", args.len());
    if args.len() < 2 {
        panic!(
            "myftw:  require one argument but get {}\n myftw: usage: ftw root_path [[-c]]",
            args.len()
        )
    }
    let mut file_statistics = FileStatisticSummary::new();
    if args.len() == 2 {
        ftw(Path::new(&args.next_back().unwrap()), &mut file_statistics);
    } else {
        ftw_chdir(Path::new(&args.nth(1).unwrap()), &mut file_statistics);
    }
    print_statistics(&file_statistics);
}

fn ftw(root: &Path, file_statistics: &mut FileStatisticSummary) {
    let meta_data = fs::symlink_metadata(root).unwrap_or_else(|error| {
        panic!(
            "line={} symlink_metadata occurs an error, {:?}, file_path={:?}",
            line!(),
            error.to_string(),
            root
        );
    });
    let file_type = meta_data.file_type().get_current_type();
    file_statistics.push(meta_data);
    if file_type == FileTypeEnum::Directory {
        let dir = root
            .read_dir()
            .unwrap_or_else(|e| panic!("read_dir occurs an error: {:?}, path: {:?}", e, root));
        dir.map(|entry_wrap| entry_wrap.unwrap()).for_each(|entry| {
            ftw(&entry.path(), file_statistics);
        });
    }
}

fn ftw_chdir(root: &Path, file_statistics: &mut FileStatisticSummary) {
    let meta_data = fs::symlink_metadata(root).unwrap_or_else(|error| {
        panic!(
            "line={} symlink_metadata occurs an error, {:?}, file_path={:?}",
            line!(),
            error.to_string(),
            root
        );
    });
    let file_type = meta_data.file_type().get_current_type();
    file_statistics.push(meta_data);
    if file_type == FileTypeEnum::Directory {
        env::set_current_dir(root).expect("set_current_dir occurs an error");
        let dir = fs::read_dir(".")
            .unwrap_or_else(|e| panic!("read_dir occurs an error: {:?}, path: {:?}", e, root));
        dir.map(|entry_wrap| entry_wrap.unwrap()).for_each(|entry| {
            ftw_chdir(&entry.path(), file_statistics);
        });
        env::set_current_dir("..").unwrap();
    }
}

fn print_statistics(statistics: &FileStatisticSummary) {
    let all_files_quantity = statistics.total();
    println!("🚀 total file count is: {}", all_files_quantity);
    let summary: &std::collections::HashMap<FileTypeEnum, Vec<fs::Metadata>> = statistics.summary();
    if summary.is_empty() {
        println!("------ no files here");
    } else {
        for file_type in &FILE_TYPES {
            match summary.get(file_type) {
                Some(files) => {
                    print_single_type_statistic(file_type, files.len(), all_files_quantity)
                }
                None => print_single_type_statistic(file_type, 0, all_files_quantity),
            };
        }
    }
}

fn print_single_type_statistic(file_type: &FileTypeEnum, quantity: usize, total: usize) {
    let mut proportion = 0.0;
    if total > 0 {
        proportion = (quantity as f64 / total as f64) * 100.0;
    }
    println!(
        "------{} file's quantity({}), proportion of total({}) is: {}%",
        file_type.convert_str(),
        quantity,
        total,
        proportion
    );
}
