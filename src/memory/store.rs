//! Agent memory — REAL session/project/task memory (Section 20 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub struct MemoryStore {
    pub session: Vec<String>,
    pub project: Vec<String>,
    pub task: Vec<String>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self { session: Vec::new(), project: Vec::new(), task: Vec::new() }
    }

    pub fn add_session(&mut self, entry: String) {
        self.session.push(entry);
    }

    pub fn add_project(&mut self, entry: String) {
        self.project.push(entry);
    }

    pub fn add_task(&mut self, entry: String) {
        self.task.push(entry);
    }

    pub fn clear_all(&mut self) {
        self.session.clear();
        self.project.clear();
        self.task.clear();
    }

    pub fn retrieve_project(&self) -> Vec<String> {
        self.project.clone()
    }
}
