use super::{AuthTokenExtractor, Response};
use crate::agent;
use crate::server::Server;
use axum::extract::Extension;
use axum::Json;
use hyper::StatusCode;
use log::{error, info};
use std::sync::Arc;

pub async fn promote_handler(
    auth_token: AuthTokenExtractor,
    server: Extension<Arc<Server>>,
) -> (StatusCode, Json<Response>) {
    if auth_token.0 != server.config.server.auth_key {
        return (
            StatusCode::UNAUTHORIZED,
            Json(Response::error("Invalid auth key".to_string())),
        );
    }

    info!("Performing promotion...");
    server.alert_client.info("Performing promotion...");

    match agent::promote(&server.config).await {
        Ok(()) => {
            info!("Promotion successful");
            server.alert_client.info("Promotion successful");

            (StatusCode::OK, Json(Response::success()))
        }
        Err(e) => {
            error!("Error promoting node: {}", e);
            server.alert_client.error(format!("Error promoting node: {}", e).as_str());

            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response::error(e.to_string())))
        }
    }
}
