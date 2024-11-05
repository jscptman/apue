mod summary;
use r::common_mod::r_os::unix::r_fs::r_file_type_etx::{FileTypeEnum, RFileTypeExt, FILE_TYPES};
use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};
use summary::FileStatisticSummary;
fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!(
            "myftw:  require one argument but get {}\n myftw: usage: ftw root_path",
            args.len()
        )
    }
    let mut unknown_files = Vec::new();
    let mut file_statistics = FileStatisticSummary::new();
    ftw(
        Path::new(&args.next_back().unwrap()),
        &mut file_statistics,
        &mut unknown_files,
    );
    print_statistics(&file_statistics);
}

fn ftw(
    root: &Path,
    file_statistics: &mut FileStatisticSummary,
    unknown_file_vec: &mut Vec<PathBuf>,
) {
    let meta_data = fs::symlink_metadata(root);
    if let Ok(meta_data) = meta_data {
        let file_type = meta_data.file_type().get_current_type();
        file_statistics.push(meta_data);
        match file_type {
            FileTypeEnum::Directory => {
                let dir = root.read_dir().unwrap_or_else(|e| {
                    panic!("read_dir occurs an error: {:?}, path: {:?}", e, root)
                });
                dir.map(|entry_wrap| entry_wrap.unwrap()).for_each(|entry| {
                    ftw(&entry.path(), file_statistics, unknown_file_vec);
                });
            }
            _ => {
                return;
            }
        };
    } else {
        unknown_file_vec.push(root.to_path_buf());
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
