mod summary;
use summary::FileStatisticSummary;
use std::{env, fs::OpenOptions, io::{self}, path::Path};
fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!(
            "myftw:  require one argument but get {}\n myftw: usage: ftw root_path",
            args.len()
        )
    }
    let result =  ftw(Path::new(&args.next_back().unwrap())).unwrap();
}
fn ftw(root: &Path) -> io::Result<FileStatisticSummary> {
    let file = OpenOptions::new().read(true).open(root)?;
    Ok(FileStatisticSummary::new())
}