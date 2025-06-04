mod parser;
mod types;
mod engine;
mod context;
mod logger;
mod steps;

use clap::{Parser as ClapParser, Subcommand};
use parser::load_workflow;
use engine::run_workflow;

#[derive(ClapParser)]
#[command(name = "rustom8", version = "0.02", about = "A tiny rust workflow engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        file: String,
    },
}

fn main() {
    // Initialize app logger:
    logger::init_logger().expect("Failed to initialize logger");
    
    // Initialize the cli instance:
    let cli = Cli::parse();

    // Define command use-cases and their actions:
    match cli.command {
        Commands::Run { file } => {
            let workflow = load_workflow(&file).expect("Failed to load workflow file");
            run_workflow(workflow);
        }
    }
}
