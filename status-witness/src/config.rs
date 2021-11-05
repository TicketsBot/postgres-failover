use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub auth_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub database: String,
    pub allowed_hosts: Vec<cidr::Ipv4Cidr>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub ip: String,
    pub port: u16,
    pub priority: u8,
}

impl Config {
    pub fn load() -> Config {
        let data = fs::read("config.toml").expect("failed to read config.toml");
        toml::from_slice(data.as_slice()).expect("failed to parse config")
    }
}
