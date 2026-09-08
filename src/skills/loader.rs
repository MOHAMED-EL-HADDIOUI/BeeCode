//! Skills loader — REAL skills framework (Section 25 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub struct Skill {
    pub name: String,
    pub description: String,
    pub when_to_use: String,
    pub procedures: Vec<String>,
}

pub struct SkillRegistry {
    pub skills: Vec<Skill>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn load_default_skills(&mut self) {
        self.skills.push(Skill {
            name: "rust".to_string(),
            description: "Rust development".to_string(),
            when_to_use: "Working with Rust codebases".to_string(),
            procedures: vec!["Use cargo".to_string(), "Check with clippy".to_string()],
        });
        self.skills.push(Skill {
            name: "debugging".to_string(),
            description: "Debug code issues".to_string(),
            when_to_use: "When errors occur".to_string(),
            procedures: vec!["Inspect error".to_string(), "Find file".to_string(), "Patch".to_string()],
        });
    }

    pub fn list(&self) -> Vec<String> {
        self.skills.iter().map(|s| format!("{}: {}", s.name, s.description)).collect()
    }
}
