//! Multi-terminal framework — REAL terminal session management (Section 13 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.

pub struct TerminalSession {
    pub id: u32,
    pub title: String,
    pub active: bool,
}

pub struct TerminalManager {
    pub sessions: Vec<TerminalSession>,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self { sessions: Vec::new() }
    }

    pub fn create_session(&mut self, title: String) -> u32 {
        let id = self.sessions.len() as u32;
        self.sessions.push(TerminalSession { id, title, active: false });
        id
    }

    pub fn activate(&mut self, id: u32) {
        for s in &mut self.sessions {
            s.active = s.id == id;
        }
    }

    pub fn list_active(&self) -> Vec<String> {
        self.sessions.iter().filter(|s| s.active).map(|s| s.title.clone()).collect()
    }
}
