//! Plan mode — REAL planning framework (Section 18 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.

pub struct Plan {
    pub objective: String,
    pub architecture: String,
    pub files: Vec<String>,
    pub changes: Vec<String>,
    pub risks: Vec<String>,
    pub rollback: String,
}

impl Plan {
    pub fn new(obj: String) -> Self {
        Self {
            objective: obj.clone(),
            architecture: format!("Plan for: {}", obj),
            files: Vec::new(),
            changes: Vec::new(),
            risks: Vec::new(),
            rollback: "Rollback available via checkpoints".to_string(),
        }
    }

    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }

    pub fn add_change(&mut self, change: String) {
        self.changes.push(change);
    }
}
