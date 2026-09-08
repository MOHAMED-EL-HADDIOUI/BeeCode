//! Phase execution tracking — REAL 9-phase framework (Section 45 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub enum Phase {
    Phase1, // Audit / Architecture
    Phase2, // Async runtime / Process / Agent state
    Phase3, // WorkspaceIndexer / Search / ContextEngine
    Phase4, // Multi-provider / Streaming / Routing
    Phase5, // Git / Checkpoints / Plan / Review / Self-healing
    Phase6, // Plugins / MCP / Skills / Workflows / Memory
    Phase7, // TUI redesign / Branding / Metrics
    Phase8, // Headless / CI / Editor / Remote
    Phase9, // Releases / Installer / Security / Benchmarks / Final cleanup
}

pub struct PhaseTracker {
    pub completed: Vec<u32>,
    pub current: Phase,
    pub buildable: bool,
}

impl PhaseTracker {
    pub fn new() -> Self {
        Self {
            completed: Vec::new(),
            current: Phase::Phase1,
            buildable: true,
        }
    }

    pub fn complete_phase(&mut self, phase_num: u32) {
        self.completed.push(phase_num);
    }

    pub fn is_complete(&self, phase: u32) -> bool {
        self.completed.contains(&(phase as u32))
    }

    pub fn report(&self) -> String {
        format!("Completed phases: {:?} | Current: Phase{:?} | Buildable: {}",
            self.completed, self.current as u32, self.buildable)
    }
}
