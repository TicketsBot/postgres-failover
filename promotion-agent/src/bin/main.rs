use log::info;
use promotion_agent::server::Server;
use promotion_agent::{Config, Result};
use discord_alerts::AlertClient;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::load();
    let alert_client = AlertClient::new("Promotion Agent", config.discord_webhook.clone()).expect("failed to create alert client");

    let server = Server::new(config, alert_client);

    info!("Starting server...");
    server.start().await
}
