use std::sync::Arc;

use crate::services::*;
use crate::sqlx_client::*;

pub mod proposals;
pub mod voters;
pub mod votes;

#[derive(Clone)]
pub struct Context {
    pub services: Arc<Services>,
    pub sqlx_client: SqlxClient,
}
