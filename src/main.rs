use std::error::Error;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod api;
mod bot;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    info!("Getting config...");
    let config = config::Config::new();

    info!("Starting mr bored_bot...");

    bot::run(config).await
}
