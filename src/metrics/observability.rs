//! Metrics / Observability — REAL tracking (Section 29 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub struct Metrics {
    pub startup_time_ms: u64,
    pub model_latency_ms: u64,
    pub tool_latency_ms: u64,
    pub render_latency_ms: u64,
    pub memory_usage_bytes: u64,
    pub active_tasks: u32,
    pub retries: u32,
    pub failures: u32,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            startup_time_ms: 0,
            model_latency_ms: 0,
            tool_latency_ms: 0,
            render_latency_ms: 0,
            memory_usage_bytes: 0,
            active_tasks: 0,
            retries: 0,
            failures: 0,
        }
    }

    pub fn record_startup(&mut self, ms: u64) {
        self.startup_time_ms = ms;
    }

    pub fn record_model_latency(&mut self, ms: u64) {
        self.model_latency_ms = ms;
    }

    pub fn increment_retry(&mut self) {
        self.retries += 1;
    }
}
