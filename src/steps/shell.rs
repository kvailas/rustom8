use anyhow::anyhow;
use tracing::{debug, error};
use crate::context::render_template;
use crate::types::{Context, ShellOutput, Step, StepExecutor, StepKind, StepOutput};

pub struct ShellStep;

impl StepExecutor for ShellStep {
    fn execute(&self, step_id: u16, step: &Step, ctx: &mut Context) -> anyhow::Result<()> {
        if let StepKind::Shell { command } = &step.kind {
            debug!("Raw shell command: {:?}", command);
            
            let rendered_command = render_template(command, ctx).expect("Invalid shell step template");
            
            let runtime_output = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to execute shell command");
            
            let mut output = StepOutput::Shell(ShellOutput {
                command: rendered_command,
                stdout: String::from_utf8_lossy(&runtime_output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&runtime_output.stderr).to_string(),
                status: runtime_output.status.code().unwrap_or(-1),
            });
            
            ctx.set_step_output(
                step_id,
                &mut output
            );
            
            Ok(())
        } else {
            error!("Shell step is invalid. Invalid step kind.");
            
            Err(anyhow!("Invalid Shell step kind"))
        }
    }
}