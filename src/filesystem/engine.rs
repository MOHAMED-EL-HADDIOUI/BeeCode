//! Safe file system engine — REAL atomic writes and safe edits (Section 14 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.
use std::fs;

pub fn safe_write(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

pub fn atomic_write(path: &str, content: &str) -> Result<(), std::io::Error> {
    let temp = format!("{}.tmp", path);
    fs::write(&temp, content)?;
    fs::rename(&temp, path)
}

pub fn detect_binary(path: &str) -> bool {
    fs::read(path).map(|bytes| bytes.contains(&0)).unwrap_or(false)
}
