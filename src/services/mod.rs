use crate::settings::Config;
use crate::sqlx_client::SqlxClient;

mod proposal;

pub struct Services {
    sqlx_client: SqlxClient,
}

impl Services {
    pub fn new(_config: &Config, sqlx_client: SqlxClient) -> Self {
        Self {
            sqlx_client,
        }
    }
}
