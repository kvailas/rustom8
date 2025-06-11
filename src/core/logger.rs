use std::fs::OpenOptions;
use std::io;
use tracing::Level;
use tracing_subscriber::{EnvFilter};

pub fn init_logger() -> anyhow::Result<()> {
    // Open the file in append mode
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rustom8.log")?;

    // Wrap it in a MakeWriter-compatible closure
    let make_writer = move || {
        // Re-open every time (simplest and safest for now)
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("rustom8.log")
            .unwrap();

        Box::new(file) as Box<dyn io::Write + Send>
    };

    tracing_subscriber::fmt()
        .with_writer(make_writer)
        .with_max_level(Level::DEBUG)
        .with_env_filter("debug") // or hardcode with `.with_env_filter("debug")`
        .init();

    Ok(())
}
