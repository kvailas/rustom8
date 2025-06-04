use anyhow::anyhow;
use crate::context::render_template;
use crate::types::{Context, ShellOutput, Step, StepExcecutor, StepKind, StepOutput};

pub struct ShellStep;

impl StepExcecutor for ShellStep {
    fn execute(&self, step_id: u16, step: &Step, ctx: &mut Context) -> anyhow::Result<()> {
        if let StepKind::Shell { command } = &step.kind {
            
            // todo: log original command (debug)
            
            let rendered_command = render_template(command, ctx).expect("Invalid shell step template");
            
            // todo: log rendered command (debug)
            
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to execute shell command");
            
            let mut output = StepOutput::Shell(ShellOutput {
                command: rendered_command,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                status: output.status.code().unwrap_or(-1),
            });
            
            ctx.set_step_output(
                step_id,
                &mut output
            );
            
            Ok(())
        } else {
            // todo: log failure details (error)
            
            Err(anyhow!("Invalid Shell step kind"))
        }
    }
}