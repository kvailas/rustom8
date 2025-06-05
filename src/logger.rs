use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use tracing::Level;
// use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::SubscriberBuilder;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

pub fn init_logger() -> anyhow::Result<()> {
    let make_writer = BoxMakeWriter::new(|| {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("rustom8.log")
            .expect("Failed to open log file");

        Box::new(BufWriter::new(file)) as Box<dyn Write + Send>
    });

    SubscriberBuilder::default()
        .with_max_level(Level::DEBUG)
        .with_writer(make_writer)
        .with_env_filter("debug")
        // .with_env_filter(EnvFilter::from_default_env()) // set RUST_LOG=debug, etc.
        .init();

    Ok(())
}
