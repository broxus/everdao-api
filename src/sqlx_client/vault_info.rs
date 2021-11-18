use crate::models::sqlx::VaultInfoFromDb;
use crate::sqlx_client::SqlxClient;
use std::collections::HashSet;

impl SqlxClient {
    pub async fn get_vault_info_by_vault_address(
        &self,
        vault_address: &str,
    ) -> Result<VaultInfoFromDb, anyhow::Error> {
        sqlx::query_as!(
            VaultInfoFromDb,
            r#"SELECT * FROM vault_info WHERE vault_address = $1"#,
            vault_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn get_vault_info_by_chain_id_and_eth_address(
        &self,
        chain_id: i32,
        eth_address: &str,
    ) -> Result<VaultInfoFromDb, anyhow::Error> {
        sqlx::query_as!(
            VaultInfoFromDb,
            r#"SELECT * FROM vault_info WHERE chain_id = $1 AND eth_token_address = $2"#,
            chain_id,
            eth_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn get_vault_info_by_chain_id_and_proxy(
        &self,
        chain_id: i32,
        proxy: &str,
    ) -> Result<VaultInfoFromDb, anyhow::Error> {
        sqlx::query_as!(
            VaultInfoFromDb,
            r#"SELECT * FROM vault_info WHERE chain_id = $1 AND proxy = $2"#,
            chain_id,
            proxy
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn get_old_vaults(&self) -> Result<HashSet<VaultInfoFromDb>, anyhow::Error> {
        let res: Vec<VaultInfoFromDb> =
            sqlx::query_as!(VaultInfoFromDb, r#"SELECT * FROM vault_info"#)
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default();
        Ok(res.into_iter().collect::<HashSet<_>>())
    }

    pub async fn insert_vaults_info(&self, new_vaults_info: Vec<VaultInfoFromDb>) {
        for vault in new_vaults_info {
            if let Err(e) = sqlx::query!(r#"INSERT INTO vault_info (vault_address, ton_token_address, eth_token_address,
                        ton_currency_scale, ton_currency, chain_id, proxy) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            vault.vault_address,
            vault.ton_token_address,
            vault.eth_token_address,
            vault.ton_currency_scale,
            vault.ton_currency,
            vault.chain_id,
            vault.proxy).execute(&self.pool).await {
                log::error!("{}", e);
            }
        }
    }
}
