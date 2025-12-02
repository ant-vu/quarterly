use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Count non-empty lines in a UTF-8 text file.
pub fn count_lines_in_file<P: AsRef<std::path::Path>>(path: P) -> Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut count: usize = 0;
    for line in reader.lines() {
        let s = line?;
        if !s.trim().is_empty() {
            count += 1;
        }
    }
    Ok(count)
}
