use tracing::Level;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::fmt::SubscriberBuilder;
use tracing_subscriber::EnvFilter;
use std::fs::OpenOptions;

pub fn init_logger() -> anyhow::Result<()> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("rustom8.log")?;

    let file_writer = BoxMakeWriter::new(log_file);

    SubscriberBuilder::default()
        .with_max_level(Level::DEBUG)
        .with_writer(file_writer)
        .with_env_filter(EnvFilter::from_default_env()) // allow RUST_LOG override
        .init();

    Ok(())
}