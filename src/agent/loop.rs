//! AgentLoop — REAL agent state machine with bounded retries (Section 7 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

#[derive(Clone, PartialEq, Eq, Debug)]
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

pub struct AgentLoop {
    pub state: AgentState,
    pub retry_count: u32,
    pub max_retries: u32,
    pub task_completed: bool,
}

impl AgentLoop {
    pub fn new() -> Self {
        Self {
            state: AgentState::IDLE,
            retry_count: 0,
            max_retries: 3,
            task_completed: false,
        }
    }

    pub fn step(&mut self, tool_result: Option<Result<String, String>>) -> AgentState {
        match self.state {
            AgentState::IDLE => {
                self.state = AgentState::THINKING;
            }
            AgentState::THINKING => {
                self.state = AgentState::PLANNING;
            }
            AgentState::PLANNING => {
                self.state = AgentState::EXECUTING;
            }
            AgentState::EXECUTING => {
                self.state = AgentState::WAITING_TOOL;
            }
            AgentState::WAITING_TOOL => {
                match tool_result {
                    Some(Ok(_)) => {
                        self.retry_count = 0;
                        self.state = AgentState::VALIDATING;
                    }
                    Some(Err(_)) => {
                        self.retry_count += 1;
                        if self.retry_count >= self.max_retries {
                            self.state = AgentState::FAILED;
                        } else {
                            self.state = AgentState::RECOVERING;
                        }
                    }
                    None => {
                        // still waiting
                    }
                }
            }
            AgentState::VALIDATING => {
                self.task_completed = true;
                self.state = AgentState::COMPLETED;
            }
            AgentState::RECOVERING => {
                self.state = AgentState::EXECUTING;
            }
            _ => {}
        }
        self.state.clone()
    }

    pub fn reset(&mut self) {
        self.state = AgentState::IDLE;
        self.retry_count = 0;
        self.task_completed = false;
    }
}
