use anyhow::{bail, Result};
use endio_bit::BEBitWriter;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn main() -> Result<()> {
    let mut file_in = BufReader::new(File::open("input.txt").expect("open failed"));

    let mut read_buf = Vec::<u8>::new();
    let file_out = File::create("output")?;
    let mut buf_writer = BEBitWriter::new(file_out);
    let mut write_buf = Vec::<u8>::new();

    while file_in
        .read_until(b'\n', &mut read_buf)
        .expect("read_until failed")
        != 0
    {
        let s = String::from_utf8(read_buf).expect("from_utf8 failed");
        for c in s.chars() {
            match c {
                '1' => buf_writer.write_bit(true).unwrap(),
                '0' => buf_writer.write_bit(false).unwrap(),
                '\n' | '\r' => {}
                _ => bail!(format!("Unexpected input: {}!", c.escape_debug())),
            }
        }
        buf_writer.write(&write_buf)?;
        write_buf.clear();

        // this returns the ownership of the read data to buf
        // there is no allocation
        read_buf = s.into_bytes();
        read_buf.clear();
    }
    Ok(())
}
