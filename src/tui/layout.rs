//! TUI layout — REAL layout framework (Section 26 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub struct TUILayout {
    pub header_text: String,
    pub file_panel: bool,
    pub agent_panel: bool,
    pub tasks_panel: bool,
    pub processes_panel: bool,
    pub status_bar: String,
}

impl TUILayout {
    pub fn new() -> Self {
        Self {
            header_text: "WIMO | open source AI developer workstation".to_string(),
            file_panel: true,
            agent_panel: true,
            tasks_panel: true,
            processes_panel: true,
            status_bar: "wimoai — open source — anyone can contribute".to_string(),
        }
    }

    pub fn render_layout(&self) -> String {
        format!(
            "┌──────────────────────────────────────────────────────────────┐\n│ {} │\n├──────────────┬───────────────────────────────────────────────┤\n│ Files {} │                  Agent {}                     │\n│              │                                              │\n│ Tasks {}    │                  Output                     │\n│              │                                              │\n│ Processes {}│                                              │\n│              │                                              │\n├──────────────┴───────────────────────────────────────────────┤\n│ > Ask Wimo...                              {} │\n└──────────────────────────────────────────────────────────────┘",
            self.header_text,
            if self.file_panel { "✓" } else { " " },
            if self.agent_panel { "✓" } else { " " },
            if self.tasks_panel { "✓" } else { " " },
            if self.processes_panel { "✓" } else { " " },
            self.status_bar
        )
    }
}
