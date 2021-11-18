use std::collections::HashMap;

use crate::models::sqlx::GraphqlEndPoint;
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn get_all_graphql_endpoints(&self) -> HashMap<i32, String> {
        let end_points = sqlx::query_as!(GraphqlEndPoint, r#"SELECT * FROM graphql_endpoints"#).fetch_all(&self.pool).await.unwrap_or_default();
        end_points.into_iter().map(|x| (x.chain_id, x.url)).collect::<HashMap<_, _>>()
    }
}