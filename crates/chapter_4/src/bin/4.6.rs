use std::{
    env,
    fs::{self, OpenOptions},
    io::Result,
    os::unix::fs::FileExt,
    path::Path,
};

const ARGS_COUNT: usize = 3;
fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != ARGS_COUNT {
        panic!("❗️arguments count error, except two found {}", args.len())
    }
    let write_buffer = b"hello";
    let hole_file = Path::new(&args[1]);
    let cp_file = Path::new(&args[2]);
    make_hole_file(hole_file, &write_buffer[..]).expect("function make_hole_file makes an error");
    cp_no_hole_file(hole_file, cp_file).expect("function cp_no_hole_file makes an error");
}

fn make_hole_file(path: &Path, buffer: &[u8]) -> Result<()> {
    let file: std::fs::File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all_at(buffer, 102400)?;
    Ok(())
}

fn cp_no_hole_file(source_path: &Path, target_path: &Path) -> Result<()> {
    let content = fs::read(source_path)?;
    let mut content_no_zero = Vec::new();
    for c in &content {
        if *c != 0 {
            content_no_zero.push(*c);
        }
    }
    fs::write(target_path, content_no_zero)?;
    Ok(())
}
