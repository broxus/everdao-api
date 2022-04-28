use std::net::SocketAddr;

use config::{Config as RawConfig, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub healthcheck_addr: SocketAddr,
    pub database_url: String,
    pub db_pool_size: u32,
    pub states_rpc_endpoint: String,

    pub brokers: String,
    pub kafka_topic: String,
    pub kafka_group_id: String,
    pub kafka_client_id: String,

    pub indexer_prod_url: String,
    pub indexer_test_url: String
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = RawConfig::new();
        s.merge(Environment::new())?;

        s.try_into()
    }
}
