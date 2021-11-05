use crate::{Config, Result};
use axum::handler::post;
use axum::{AddExtensionLayer, Router};
use std::sync::Arc;

pub struct Server {
    pub config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    pub async fn start(self) -> Result<()> {
        let address = self.config.server.address.parse()?;

        let server = Arc::new(self);

        let app = Router::new()
            .route("/check", post(super::check_handler))
            .layer(AddExtensionLayer::new(server));

        hyper::Server::bind(&address)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
