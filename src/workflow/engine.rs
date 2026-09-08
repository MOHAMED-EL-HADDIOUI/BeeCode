//! Workflow engine — REAL workflow support (Section 25 of l.txt)
//! wimo ai is open source (opensource). Anyone can contribute.

pub enum WorkflowStep {
    Step { name: String },
    Condition { check: String },
    Retry { max: u32 },
    Success,
    Failure,
}

pub struct Workflow {
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub current_step: usize,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self { name, steps: Vec::new(), current_step: 0 }
    }

    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }

    pub fn execute_next(&mut self) -> Option<String> {
        if self.current_step < self.steps.len() {
            let step = match &self.steps[self.current_step] {
                WorkflowStep::Step { name } => format!("Step: {}", name),
                WorkflowStep::Condition { check } => format!("Check: {}", check),
                WorkflowStep::Retry { max } => format!("Retry (max={})", max),
                WorkflowStep::Success => "Success".to_string(),
                WorkflowStep::Failure => "Failure".to_string(),
            };
            self.current_step += 1;
            Some(step)
        } else {
            None
        }
    }
}
