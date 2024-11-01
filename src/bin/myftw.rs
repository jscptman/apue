mod summary;
use std::{
    env,
    fs::OpenOptions,
    io::{self},
    path::Path,
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
    let summary = ftw(Path::new(&args.next_back().unwrap())).unwrap();
}
fn ftw(root: &Path) -> io::Result<FileStatisticSummary> {
    let mut summary = FileStatisticSummary::new();
    let file = OpenOptions::new()
        .read(true)
        .open(root)
        .unwrap_or_else(|e| panic!("ftw occurs an error: {:?},argument root is: {:?}", e, root));
    Ok(summary)
}
