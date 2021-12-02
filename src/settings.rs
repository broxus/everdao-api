use std::net::{SocketAddr};

use config::{Config as RawConfig, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub healthcheck_addr: SocketAddr,
    pub database_url: String,
    pub db_pool_size: u32,
    pub kafka_settings_path: String,
    pub states_rpc_endpoint: String,
    pub kafka_group_id: String,
    pub kafka_client_id: String,
    pub kafka_user: String,
    pub kafka_password: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = RawConfig::new();
        s.merge(Environment::new())?;

        s.try_into()
    }
}
