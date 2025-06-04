use handlebars::Handlebars;
use once_cell::sync::Lazy;
use crate::types::{Context, StepContext, StepOutput};

pub static HANDLEBARS: Lazy<Handlebars<'static>> = Lazy::new(|| {
    let h = Handlebars::new();
    h
});

pub fn render_template(input: &str, ctx: &Context) -> anyhow::Result<String> {
    // Convert context into a serde_json::Value
    let data = serde_json::to_value(ctx)?; // Requires #[derive(Serialize)] on Context

    let rendered = HANDLEBARS.render_template(input, &data)?;
    Ok(rendered)
}

impl Context {
    pub fn set(&mut self, step_id: u16, output: &mut StepOutput) {
        self.steps.insert(step_id, StepContext { output: output.clone() });
    }
    
    // pub fn get(&mut self, step_id: u16) -> StepContext {
    //     self.steps.get(&step_id).unwrap().clone()
    // }

    pub fn set_step_output(&mut self, step_id: u16, output: &mut StepOutput) {
        self.steps.insert(step_id, StepContext { output: output.clone() });
    }
}