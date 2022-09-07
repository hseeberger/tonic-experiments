mod server;

use anyhow::{Context, Result};
use configured::Configured;
use serde::Deserialize;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: server::Config,
}

pub fn init_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .try_init()
        .context("Cannot initialize tracing subscriber")
}

pub async fn run() -> Result<()> {
    // Load configuration
    let config = Config::load().context("Cannot load configuration")?;
    debug!(message = "Starting", config = debug(&config));

    // Run the server
    let server = server::run(&config.server);
    info!("Server up and running at {:?}", config.server);
    server.await
}
