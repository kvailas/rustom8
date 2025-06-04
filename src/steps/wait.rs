use std::thread;
use std::time::Duration;
use anyhow::anyhow;
use crate::context::render_template;
use crate::types::{Context, Step, StepExcecutor, StepKind, StepOutput, WaitOutput};

pub struct WaitStep;

impl StepExcecutor for WaitStep {
    fn execute(&self, step_id: u16, step: &Step, ctx: &mut Context) -> anyhow::Result<()> {
        if let StepKind::Wait { duration_ms } = &step.kind {
            let duration = String::from(duration_ms.to_string());
            let mut step_output = StepOutput::Wait(WaitOutput { duration: duration_ms.clone(), completed: false });

            // todo: log original duration set (debug)

            let duration_ms: u64 = render_template(&duration, ctx).expect("Invalid wait step template")
                .parse::<u64>().expect("Failed to parse duration number from renderer");

            // todo: log wit step initiated (info)

            ctx.set(step_id, &mut step_output);

            thread::sleep(Duration::from_millis(duration_ms));
            
            ctx.set(step_id, &mut step_output);

            // todo: log finished (info)

            Ok(())
        } else {
            // todo: log failure details (error)

            Err(anyhow!("Invalid Wait step kind"))
        }
    }
}