use std::collections::HashSet;
use std::str::FromStr;
use std::time::Duration;

use chrono::Utc;
use reqwest::{Client, Response, Url};
use tokio::time::sleep;

use crate::graphql_client::EVMGraphqlClient;
use crate::models::reqwest::requests::CurrencyInfoRequest;
use crate::models::reqwest::responses::{CurrencyInfoResponse, TokensInfoResponse};
use crate::models::sqlx::VaultInfoFromDb;
use crate::sqlx_client::SqlxClient;

const CURRENCY_INFO: &str = "/v1/bridge_currency_info";
const TOKENS_INFO_GITHUB: &str =
    "https://raw.githubusercontent.com/broxus/bridge-assets/master/main.json";

#[derive(Clone)]
pub struct ReqwestClient {
    tonswap_indexer_url: String,
    client: Client,
}

impl ReqwestClient {
    pub fn new(tonswap_indexer_url: String) -> Self {
        let client = Client::new();
        ReqwestClient {
            tonswap_indexer_url,
            client,
        }
    }

    pub async fn get_currency_info(
        &self,
        currency_address: &str,
        timestamp: i32,
    ) -> CurrencyInfoResponse {
        let body = CurrencyInfoRequest {
            timestamp_block: timestamp,
            currency_address: currency_address.to_string(),
        };
        let body = serde_json::to_string(&body).unwrap();
        let url = format!("{}{}", self.tonswap_indexer_url, CURRENCY_INFO);
        loop {
            let res: Response = match self
                .client
                .post(Url::from_str(&url).unwrap())
                .body(body.clone())
                .send()
                .await
            {
                Ok(ok) => ok,
                Err(e) => {
                    log::error!(
                        "currency_address {} timestamp_block {} reqwest error {}",
                        currency_address,
                        timestamp,
                        e
                    );
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            let res: Result<CurrencyInfoResponse, anyhow::Error> =
                res.json().await.map_err(anyhow::Error::new);
            match res {
                Ok(ok) => return ok,
                Err(e) => {
                    log::error!(
                        "currency_address {} timestamp_block {} json error {}",
                        currency_address,
                        timestamp,
                        e
                    );
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    }
    pub async fn get_vault_info(
        &self,
        graphql_client: EVMGraphqlClient,
        sqlx_client: SqlxClient,
    ) -> HashSet<VaultInfoFromDb> {
        loop {
            let end_points = sqlx_client.get_all_graphql_endpoints().await;
            let timestamp_now = Utc::now().timestamp() as i32;
            let token_list: TokensInfoResponse = match reqwest::get(TOKENS_INFO_GITHUB).await {
                Ok(ok) => match ok.json().await {
                    Ok(ok) => ok,
                    Err(e) => {
                        log::error!("{}", e);
                        sleep(Duration::from_secs(10)).await;
                        continue;
                    }
                },
                Err(e) => {
                    log::error!("{}", e);
                    sleep(Duration::from_secs(10)).await;
                    continue;
                }
            };
            let mut res = HashSet::new();
            for (ton_token_address, token_info) in token_list.token {
                for vault_info in token_info.vaults {
                    let end_point_url = match end_points.get(&vault_info.chain_id) {
                        None => {
                            log::error!("invalid chain_id {}", vault_info.chain_id);
                            continue;
                        }
                        Some(x) => x.clone(),
                    };
                    let eth_address = match graphql_client
                        .get_eth_address_by_vault(&vault_info.vault, &end_point_url)
                        .await
                    {
                        Ok(ok) => ok,
                        Err(_) => continue,
                    };
                    let currency_info = self
                        .get_currency_info(&ton_token_address, timestamp_now)
                        .await;
                    res.insert(VaultInfoFromDb {
                        vault_address: vault_info.vault,
                        ton_token_address: ton_token_address.clone(),
                        eth_token_address: eth_address,
                        ton_currency_scale: currency_info.currency_scale,
                        ton_currency: currency_info.currency,
                        chain_id: vault_info.chain_id,
                        proxy: token_info.proxy.clone(),
                    });
                }
            }
            return res;
        }
    }
}

pub async fn loop_update_vault_info(
    graphql_client: EVMGraphqlClient,
    sqlx_client: SqlxClient,
    reqwest_client: ReqwestClient,
) {
    loop {
        sleep(Duration::from_secs(60 * 15)).await; // 15 min
        update_vault_info(
            graphql_client.clone(),
            sqlx_client.clone(),
            reqwest_client.clone(),
        )
        .await;
    }
}

pub async fn update_vault_info(
    graphql_client: EVMGraphqlClient,
    sqlx_client: SqlxClient,
    reqwest_client: ReqwestClient,
) {
    let old_vaults_info = sqlx_client.get_old_vaults().await.unwrap_or_default();
    let new_vaults_info = reqwest_client
        .get_vault_info(graphql_client.clone(), sqlx_client.clone())
        .await
        .into_iter()
        .filter(|x| !old_vaults_info.contains(x))
        .collect::<Vec<_>>();
    sqlx_client.insert_vaults_info(new_vaults_info).await;
}

#[cfg(test)]
pub mod test {
    use sqlx::postgres::PgPoolOptions;

    use crate::graphql_client::EVMGraphqlClient;
    use crate::reqwest_client::{update_vault_info, ReqwestClient};
    use crate::sqlx_client::SqlxClient;

    #[tokio::test]
    pub async fn test_get_tokens_info() {
        env_logger::init();
        let reqwest_client =
            ReqwestClient::new("https://ton-swap-indexer-test.broxus.com".to_string());
        let graphql_client = EVMGraphqlClient::default();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres:postgres@localhost:5432/bridge_dao_indexer")
            .await
            .expect("fail pg pool");
        let sqlx_client = SqlxClient::new(pool);
        update_vault_info(graphql_client, sqlx_client, reqwest_client).await;
    }
}
