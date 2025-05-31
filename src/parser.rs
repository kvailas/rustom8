use std::fs;
use crate::types::Workflow;

pub fn load_workflow(path: &str) -> Result<Workflow, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let workflow: Workflow = toml::from_str(&content)?;
    Ok(workflow)
}