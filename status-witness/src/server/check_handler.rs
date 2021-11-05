use super::{AuthTokenExtractor, Response};
use crate::server::Server;
use crate::Config;
use axum::extract::Extension;
use axum::{extract, Json};
use hyper::StatusCode;
use log::{error, info, warn};
use serde::Deserialize;
use std::convert::TryFrom;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tokio_postgres::NoTls;

#[derive(Debug, Deserialize)]
pub struct CheckBody {
    pub address: Ipv4Addr,
    #[serde(default = "default_port")]
    pub port: u16,
}

pub async fn check_handler(
    auth_token: AuthTokenExtractor,
    body: extract::Json<CheckBody>,
    server: Extension<Arc<Server>>,
) -> (StatusCode, Json<Response>) {
    if auth_token.0 != server.config.server.auth_key {
        return (
            StatusCode::UNAUTHORIZED,
            Json(Response::error("Invalid auth key")),
        );
    }

    // Don't send our credentials to random hosts!
    if !server
        .config
        .database
        .allowed_hosts
        .iter()
        .any(|range| range.contains(&body.address))
    {
        warn!("Authorized request provided disallowed IP {}", body.address);
        return (
            StatusCode::FORBIDDEN,
            Json(Response::error("Address not allowed")),
        );
    }

    info!("Performing check on {}:{}", body.address, body.port);
    let res = try_connect(&server.config, &body.0).await;
    (StatusCode::OK, Json(res))
}

async fn try_connect(config: &Config, body: &CheckBody) -> Response {
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.user(config.database.username.as_str());
    pg_config.password(config.database.password.as_str());
    pg_config.dbname(config.database.database.as_str());

    pg_config.host(body.address.to_string().as_str());
    pg_config.port(body.port);

    pg_config.connect_timeout(Duration::from_secs(5));

    let start = Instant::now();
    match pg_config.connect(NoTls).await {
        Ok(_) => {
            let elapsed = start.elapsed();
            let millis = match usize::try_from(elapsed.as_millis()) {
                Ok(v) => v,
                Err(_) => {
                    error!("elapsed time exceeded u64");
                    usize::MAX
                }
            };

            info!("Check returned online in {}ms", millis);
            Response::check_response(true, Some(millis))
        }
        Err(e) => {
            info!("Check returned offline: {}", e);
            Response::check_response(false, None)
        }
    }
}

const fn default_port() -> u16 {
    5432
}
