//! TaskManager — central scheduler (Section 3 / Section 135-161 of prompt_ai.md)
//! wimo ai is open source (opensource). Anyone can contribute.

pub enum AgentState {
    IDLE,
    THINKING,
    PLANNING,
    EXECUTING,
    WAITING_TOOL,
    VALIDATING,
    RECOVERING,
    COMPLETED,
    FAILED,
    CANCELLED,
}

pub struct TaskManager {
    pub state: AgentState,
    pub tasks: std::collections::VecDeque<Task>,
}

pub struct Task {
    pub id: u64,
    pub kind: TaskKind,
    pub priority: u8,
    pub timeout_ms: Option<u64>,
}

pub enum TaskKind {
    AgentTask,
    ShellTask,
    SearchTask,
    IndexTask,
    TestTask,
    BuildTask,
    NetworkTask,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            state: AgentState::IDLE,
            tasks: std::collections::VecDeque::new(),
        }
    }
}
