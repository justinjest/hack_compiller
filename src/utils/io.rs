use std::fs;
use std::io::{Result, Write};

pub fn open_line_breaks(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    return contents;
}

pub fn write_file(filename: &str, contents: &str) -> Result<()> {
    let mut file = fs::File::create(format!("{}.txt", filename.to_string()))?;
    let _ = file.write_all(contents.as_bytes());
    Ok(())
}
