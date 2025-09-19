use crate::config::Config;
use tracing_subscriber::prelude::*; // <-- brings `.with(...)` into scope
use tracing_subscriber::{fmt, EnvFilter};

pub fn init(_cfg: &Config) {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,actix_web=info".into());
    tracing_subscriber::registry()
        .with(EnvFilter::new(filter))
        .with(fmt::layer())
        .init();
}
