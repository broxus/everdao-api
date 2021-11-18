use crate::settings::Config;
use crate::sqlx_client::SqlxClient;

mod staking;
mod transfers;
pub(crate) mod utils;

pub struct Services {
    sqlx_client: SqlxClient,
    // config: Config,
}

impl Services {
    pub fn new(_config: &Config, sqlx_client: SqlxClient) -> Self {
        Self {
            sqlx_client,
            // config: config.clone(),
        }
    }
}
