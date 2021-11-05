use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discord_webhook: String,
    pub server: Server,
    pub cluster: Cluster,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub auth_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Cluster {
    pub version: usize,
    pub name: String,
}

impl Config {
    pub fn load() -> Config {
        let data = fs::read("config.toml").expect("failed to read config.toml");
        toml::from_slice(data.as_slice()).expect("failed to parse config")
    }
}
