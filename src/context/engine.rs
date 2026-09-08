//! ContextEngine — REAL progressive context assembly (Section 6 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub struct ContextEngine {
    pub budget: usize,
    pub selected: Vec<ContextItem>,
}

pub struct ContextItem {
    pub source: String,
    pub content: String,
    pub tokens_estimated: usize,
    pub priority: u8,
}

impl ContextEngine {
    pub fn new(budget: usize) -> Self {
        Self { budget, selected: Vec::new() }
    }

    pub fn assemble_progressive(&mut self, task: &str, files: &[String], errors: &[String]) {
        self.selected.clear();
        // 1. task
        self.add_item("task".to_string(), format!("Task: {}", task), 10, 100);
        // 2. relevant files
        for f in files.iter().take(5) {
            self.add_item("file".to_string(), f.clone(), 5, 80);
        }
        // 3. errors
        for e in errors.iter().take(3) {
            self.add_item("error".to_string(), e.clone(), 3, 90);
        }
    }

    fn add_item(&mut self, source: String, content: String, tokens: usize, priority: u8) {
        let current_tokens: usize = self.selected.iter().map(|i| i.tokens_estimated).sum();
        if current_tokens + tokens <= self.budget {
            self.selected.push(ContextItem {
                source,
                content,
                tokens_estimated: tokens,
                priority,
            });
        }
    }

    pub fn build(&self) -> String {
        self.selected.iter().map(|i| format!("[{}] {}", i.source, i.content)).collect::<Vec<_>>().join("\n")
    }
}
