use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Main base types:

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    #[serde(flatten)]
    pub kind: StepKind,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")] // This makes `type = "...""` drive the enum
pub enum StepKind {
    Shell {
        command: String,
    },
    Wait {
        duration_ms: u64,
    },
    Print {
        message: String,
    },
    // Future: Http { url: String, method: String, ... }
}

// Context related types:

#[derive(Debug, Clone, Serialize)]
pub enum StepOutput {
    Print(PrintOutput),
    Wait(WaitOutput),
    Shell(ShellOutput),
}

#[derive(Debug, Clone, Serialize)]
pub struct PrintOutput {
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WaitOutput {
    pub duration: u64,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShellOutput {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct StepContext {
    pub output: StepOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct Context {
    pub steps: HashMap<u16, StepContext>,
}