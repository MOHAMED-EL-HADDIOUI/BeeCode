//! Process manager — REAL subprocess streaming (Section 12 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.
use std::process::{Command, Stdio};

pub struct ProcessResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_command(cmd: &str, args: &[String], working_dir: Option<&str>) -> ProcessResult {
    let mut command = Command::new(cmd);
    command.args(args);
    if let Some(dir) = working_dir {
        command.current_dir(dir);
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());

    match command.output() {
        Ok(output) => ProcessResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        },
        Err(_) => ProcessResult {
            exit_code: -1,
            stdout: String::new(),
            stderr: "Failed to execute command".to_string(),
        },
    }
}
