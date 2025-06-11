use std::thread;
use std::time::Duration;
use anyhow::anyhow;
use tracing::{error};
use crate::core::context::render_template;
use crate::core::types::{Context, Step, StepExecutor, StepKind, StepOutput, WaitOutput};

pub struct WaitStep;

impl StepExecutor for WaitStep {
    fn execute(&self, step_id: u16, step: &Step, ctx: &mut Context) -> anyhow::Result<()> {
        if let StepKind::Wait { duration_ms } = &step.kind {
            let duration = String::from(duration_ms.to_string());
            let mut step_output = StepOutput::Wait(WaitOutput { duration: duration_ms.clone(), completed: false });
            
            let duration_ms: u64 = render_template(&duration, ctx).expect("Invalid wait step template")
                .parse::<u64>().expect("Failed to parse duration number from renderer");

            ctx.set(step_id, &mut step_output);

            thread::sleep(Duration::from_millis(duration_ms));

            if let StepOutput::Wait(step_output) = &mut step_output {
                step_output.completed = true;
            }
            ctx.set(step_id, &mut step_output);
            
            Ok(())
        } else {
            error!("Wait step is invalid. Invalid step kind.");

            Err(anyhow!("Invalid Wait step kind"))
        }
    }
}