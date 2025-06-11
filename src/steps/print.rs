use tracing::{debug, error};
use crate::core::context::render_template;
use crate::core::types::{Context, PrintOutput, Step, StepExecutor, StepKind, StepOutput};

pub struct PrintStep;

impl StepExecutor for PrintStep {
    fn execute(&self, step_id: u16, step: &Step, ctx: &mut Context) -> anyhow::Result<()> {
        if let StepKind::Print { message } = &step.kind {
            
            debug!("Raw print command: {:?}", message);
            
            let rendered_message = render_template(message, ctx)
                .expect("Invalid print step template");

            println!("-> Message: {}", &rendered_message);
            
            let mut output = StepOutput::Print(PrintOutput{
                message: rendered_message.to_string() });
            
            ctx.set(step_id, &mut output);
            
            Ok(())
        } else { 
            error!("Print step is invalid. Invalid step kind.");
            
            Err(anyhow::anyhow!("Invalid step kind"))
        }
    }
}