use sqlx::PgPool;

mod balances;
pub mod graph_data;
mod raw_transactions;
mod relay_events;
mod rounds_info;
mod stakeholders;
mod transactions;
mod transfers;
mod unknown_user_keys;
mod user_keys;
mod vault_info;
mod graphql_endpoints;

#[derive(Clone)]
pub struct SqlxClient {
    pool: PgPool,
}

impl SqlxClient {
    pub fn new(pool: PgPool) -> SqlxClient {
        SqlxClient { pool }
    }
}
