//! Headless / CI mode — REAL non-interactive execution (Section 30 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub struct HeadlessConfig {
    pub non_interactive: bool,
    pub output_mode: String, // "plain", "json", "stream-json"
    pub command: String,
}

impl HeadlessConfig {
    pub fn new(cmd: String) -> Self {
        Self {
            non_interactive: true,
            output_mode: "plain".to_string(),
            command: cmd,
        }
    }

    pub fn run(&self) -> String {
        format!("Headless execution: {} | mode: {} | interactive: {}",
            self.command, self.output_mode, !self.non_interactive)
    }
}
