use std::fs::File;
use std::io::{BufReader, Result, BufRead};

pub fn read_lines_from_file(file_name: &str) -> Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?)
    }
    Ok(lines)
}