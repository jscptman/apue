use std::io::{self, Cursor, Read, Write};

fn main() -> io::Result<()> {
    let buffer = [0; 100];
    let mut cursor = Cursor::new(buffer);
    write!(&mut cursor, "Hello, world!")?;
    cursor.set_position(0);
    let mut read_buffer = [0; 100];
    let _n = cursor.read(&mut read_buffer)?;
    println!(
        "Read from memory: {:?}",
        String::from_utf8_lossy(
            &read_buffer
                .into_iter()
                .filter(|c| { *c > 0 })
                .collect::<Vec<u8>>()
        )
    );
    Ok(())
}
