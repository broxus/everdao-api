use std::sync::Arc;

use crate::services::*;
use crate::sqlx_client::*;

pub use self::proposals::*;
pub use self::votes::*;

pub mod proposals;
pub mod votes;

#[derive(Clone)]
pub struct Context {
    pub services: Arc<Services>,
    pub sqlx_client: SqlxClient,
}
