use crate::{Config, Result};
use axum::handler::post;
use axum::{AddExtensionLayer, Router};
use std::sync::Arc;
use discord_alerts::AlertClient;

pub struct Server {
    pub config: Config,
    pub alert_client: AlertClient,
}

impl Server {
    pub fn new(config: Config, alert_client: AlertClient) -> Self {
        Server { config, alert_client }
    }

    pub async fn start(self) -> Result<()> {
        let address = self.config.server.address.parse()?;

        let server = Arc::new(self);

        let app = Router::new()
            .route("/promote", post(super::promote_handler))
            .layer(AddExtensionLayer::new(server));

        hyper::Server::bind(&address)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
