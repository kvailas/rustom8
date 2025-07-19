use std::collections::HashMap;
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use tracing::debug;
use crate::core::types::{Context, StepContext, StepOutput};

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
    pub fn new() -> Self {
        Context {
            steps: HashMap::new()
        }
    }

    pub fn set(&mut self, step_id: u16, output: &mut StepOutput) {
        self.steps.insert(step_id, StepContext { output: output.clone() });
        
        debug!("Context::set #{} -> {}",
            step_id,
            serde_json::to_value(&output).expect("Failed to serialize step output")
        );
    }
    
    // pub fn get(&mut self, step_id: u16) -> StepContext {
    //     self.steps.get(&step_id).unwrap().clone()
    // }
}

#[cfg(test)]
mod tests {
    use crate::core::types::PrintOutput;
    use super::*;
    // use crate::core::types::{WaitOutput}; // or mock definitions if needed

    #[test]
    fn test_render_static_text() {
        let ctx = Context::new();
        let out = render_template("Hello", &ctx).unwrap();
        assert_eq!(out, "Hello");
    }

    #[test]
    fn test_render_with_variable() {
        let mut ctx = Context::new();
        ctx.set(
            1,
            &mut StepOutput::Print(PrintOutput{message: "Hello!".to_string()})
        );
        let out = render_template("{{steps.1.output.Print.message}}", &ctx).unwrap();
        assert_eq!(out, "Hello!");
    }

    #[test]
    fn test_set_context_output() {
        let mut ctx = Context::new();
        ctx.set(
            1,
            &mut StepOutput::Print(PrintOutput{message: "Hello 1!".to_string()})
        );
        ctx.set(
            2,
            &mut StepOutput::Print(PrintOutput{message: "Hello 2!".to_string()})
        );
        assert!(ctx.steps.contains_key(&1)); // should contain step 1
        assert!(ctx.steps.contains_key(&2)); // should contain step 2
        assert!(!ctx.steps.contains_key(&3)); // shouldn't contain step 3 (or upwards)
    }

    #[test]
    fn test_invalid_template_returns_error() {
        let ctx = Context::new();
        let out = render_template("{{invalid", &ctx);
        assert!(out.is_err());
    }
}