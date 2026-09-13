//! Git Intelligence — REAL basic git integration (Section 16 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.
use std::process::Command;

pub struct GitInfo {
    pub branch: String,
    pub status: String,
    pub changed_files: Vec<String>,
}

pub fn get_git_info(path: &str) -> Option<GitInfo> {
    let branch = Command::new("git")
        .args(["-C", path, "branch", "--show-current"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string());

    let status_output = Command::new("git")
        .args(["-C", path, "status", "--short"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string());

    let changed_files: Vec<String> = status_output.as_ref()
        .map(|s| s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect())
        .unwrap_or_default();

    Some(GitInfo {
        branch,
        status: status_output.unwrap_or_default(),
        changed_files,
    })
}
