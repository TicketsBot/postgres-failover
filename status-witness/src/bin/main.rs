use status_witness::server::Server;
use status_witness::{Config, Result};
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::load();
    let server = Server::new(config);

    info!("Starting server...");
    server.start().await
}
