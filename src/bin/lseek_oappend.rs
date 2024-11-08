use std::{
    fs::OpenOptions,
    io::{Read, Result, Seek, Write},
    path::Path,
};

fn main() -> Result<()> {
    let mut read_buf = [0_u8; 6];
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(Path::new("docs/test.txt"))?;
    file.set_len(0)?;
    file.write_all(b"hello")?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(b" world!")?;
    file.seek(std::io::SeekFrom::Start(6))?;
    file.read_exact(&mut read_buf)?;
    println!(
        "🚀 read_buf: {}",
        String::from_utf8(read_buf.to_vec()).unwrap()
    );
    Ok(())
}
