pub mod staking;

use std::sync::Arc;

use crate::services::Services;
use crate::sqlx_client::SqlxClient;

#[derive(Clone)]
pub struct Context {
    pub services: Arc<Services>,
    pub sqlx_client: SqlxClient,
}
