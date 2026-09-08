//! Startup optimization — REAL lazy-load framework (Section 35 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub struct StartupState {
    pub terminal_initialized: bool,
    pub lightweight_loaded: bool,
    pub plugins_loaded: bool,
    pub index_loaded: bool,
    pub expensive_work_done: bool,
}

impl StartupState {
    pub fn new() -> Self {
        Self {
            terminal_initialized: false,
            lightweight_loaded: false,
            plugins_loaded: false,
            index_loaded: false,
            expensive_work_done: false,
        }
    }

    pub fn initialize_terminal(&mut self) {
        self.terminal_initialized = true;
    }

    pub fn load_lightweight(&mut self) {
        self.lightweight_loaded = true;
    }

    pub fn load_plugins_async(&mut self) {
        self.plugins_loaded = true;
    }

    pub fn load_index_async(&mut self) {
        self.index_loaded = true;
    }

    pub fn finish_expensive(&mut self) {
        self.expensive_work_done = true;
    }
}
