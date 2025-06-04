use crate::context::render_template;
use crate::types::{Context, PrintOutput, Step, StepExcecutor, StepKind, StepOutput};

pub struct PrintStep;

impl StepExcecutor for PrintStep {
    fn execute(&self, step_id: u16, step: &Step, ctx: &mut Context) -> anyhow::Result<()> {
        if let StepKind::Print { message } = &step.kind {
            // todo: log original message (debug)
            
            let rendered_message = render_template(message, ctx).expect("Invalid print step template");

            println!("-> Message: {}", &rendered_message);
            
            let mut output = StepOutput::Print(PrintOutput{ message: rendered_message.to_string() });
            
            ctx.set(step_id, &mut output);

            // todo: log print output (info)
            
            Ok(())
        } else { 
            // todo: log failure details (error)
            
            Err(anyhow::anyhow!("Invalid step kind"))
        }
    }
}