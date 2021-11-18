use crate::models::sqlx::UserKeysFromDb;
use crate::models::user_type::UserType;
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn get_user_keys(&self, user_address: &str) -> Result<UserKeysFromDb, anyhow::Error> {
        sqlx::query_as!(
            UserKeysFromDb,
            r#"SELECT * FROM user_keys WHERE user_address = $1"#,
            user_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn upsert_user_keys(
        &self,
        update_user_keys: UserKeysFromDb,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(r#"INSERT INTO user_keys (user_address, ton_pubkey, ton_pubkey_is_confirmed, eth_address, eth_address_is_confirmed, until_frozen) 
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (user_address) DO 
            UPDATE SET ton_pubkey = $2, ton_pubkey_is_confirmed = $3, eth_address = $4, eth_address_is_confirmed = $5, until_frozen = $6, updated_at = Default"#,
        update_user_keys.user_address,
        update_user_keys.ton_pubkey,
        update_user_keys.ton_pubkey_is_confirmed,
        update_user_keys.eth_address,
        update_user_keys.eth_address_is_confirmed,
        update_user_keys.until_frozen).execute(&mut tx).await?;

        #[allow(unused_must_use)]
        if update_user_keys.ton_pubkey_is_confirmed && update_user_keys.eth_address_is_confirmed {
            sqlx::query!(
                r#"UPDATE user_balances SET user_kind = $1, until_frozen = $3 WHERE user_address = $2"#,
                UserType::Relay.to_string(),
                update_user_keys.user_address,
                update_user_keys.until_frozen,
            ).execute(&mut tx).await;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_user_kind(&self, user_address: &str) -> (UserType, i32) {
        let (ton_conf, eth_conf, until_frozen) = sqlx::query!(r#"SELECT ton_pubkey_is_confirmed, eth_address_is_confirmed, until_frozen FROM user_keys WHERE user_address = $1"#, user_address).fetch_one(&self.pool).await.map(|x| (x.ton_pubkey_is_confirmed, x.eth_address_is_confirmed, x.until_frozen)).unwrap_or((false, false, 0));
        if ton_conf && eth_conf {
            (UserType::Relay, until_frozen)
        } else {
            (UserType::Ordinary, 0)
        }
    }

    pub async fn get_user_address_by_keys(
        &self,
        ton_key: &[u8],
        eth_key: &[u8],
    ) -> Result<String, anyhow::Error> {
        sqlx::query!(
            r#"SELECT user_address FROM user_keys WHERE  ton_pubkey = $1 AND eth_address = $2"#,
            ton_key,
            eth_key
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.user_address)
        .map_err(anyhow::Error::new)
    }

    pub async fn get_relay_user_address_by_key(
        &self,
        key: &[u8],
    ) -> Result<(String, Vec<u8>), anyhow::Error> {
        sqlx::query!(
            r#"SELECT user_address, ton_pubkey FROM user_keys WHERE ton_pubkey = $1 or eth_address = $1"#,
            key,
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| (x.user_address, x.ton_pubkey))
        .map_err(anyhow::Error::new)
    }
}
