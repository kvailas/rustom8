use std::{thread, time::Duration};
use std::collections::HashMap;
use crate::context::{render_template};
use crate::types::{Workflow, StepKind, Step, Context, StepOutput, WaitOutput, PrintOutput, ShellOutput};


pub fn run_workflow(wf: Workflow) {
    println!("==> Running workflow: {}\n", wf.name);
    
    let mut ctx = Context{
        steps: HashMap::new(),
    };

    for (i, step) in wf.steps.iter().enumerate() {
        let step_id: u16 = i as u16 + 1;
        run_step(step_id, step, &mut ctx)
    }
}


fn run_step(step_id: u16, step: &Step, ctx: &mut Context) {
    match &step.kind {
        StepKind::Shell { command } => {
            // todo: log original command (debug)
            
            let rendered_command = render_template(command, ctx).expect("Invalid template");

            // todo: log rendered command (info)

            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to run shell command");

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status = output.status.code().unwrap_or(-1);

            ctx.set_step_output(
                step_id,
                StepOutput::Shell(ShellOutput {command: rendered_command, stdout, stderr, status})
            )

            // todo: log step output (info)
        }

        StepKind::Wait { duration_ms } => {
            let duration_str = String::from(duration_ms.to_string());
            
            // todo: log original duration set (debug)
            
            let duration_ms: u64 = render_template(&duration_str, ctx).expect("Invalid template")
                .parse::<u64>().expect("Failed to parse duration number from renderer");

            // todo: log wait step initiated (info)

            thread::sleep(Duration::from_millis(duration_ms.clone()));

            ctx.set(step_id, StepOutput::Wait(WaitOutput {duration: duration_ms.clone(), completed: false}));

            // todo: log wait step finished (info)
        }

        StepKind::Print { message } => {
            // todo: log original message (debug)
            let rendered_message = render_template(message.as_str(), ctx).expect("Invalid template");
            
            println!("-> Message: {}", &rendered_message);

            ctx.set(step_id, StepOutput::Print(PrintOutput {message: rendered_message.to_string()}));

            // todo: log print output (info)
        }
    }
}
