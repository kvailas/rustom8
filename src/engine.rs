use std::collections::HashMap;
use tracing::debug;
use crate::steps::print::PrintStep;
use crate::steps::shell::ShellStep;
use crate::steps::wait::WaitStep;
use crate::types::{Workflow, StepKind, Step, Context, StepExecutor};

pub fn run_workflow(wf: Workflow) {
    debug!("RUN WORKFLOW: {}", wf.name);
    
    let mut ctx = Context{
        steps: HashMap::new(),
    };

    for (i, step) in wf.steps.iter().enumerate() {
        let step_id: u16 = i as u16 + 1;
        run_step(step_id, step, &mut ctx)
    }
}


fn run_step(step_id: u16, step: &Step, ctx: &mut Context) {
    
    debug!("RUN STEP: {}", step_id);
    
    match &step.kind {
        StepKind::Print { .. } => PrintStep.execute(step_id, step, ctx).expect("Print step panicked"),
        StepKind::Wait { .. } => WaitStep.execute(step_id, step, ctx).expect("Wait step panicked"),
        StepKind::Shell { .. } => ShellStep.execute(step_id, step, ctx).expect("Shell step panicked"),
    }
}
