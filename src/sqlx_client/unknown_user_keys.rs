use crate::models::address_type::UserAddressType;
use crate::models::sqlx::UnknownUserKeysFromDb;
use crate::models::user_type::UserType;
use crate::sqlx_client::SqlxClient;
use std::str::FromStr;

impl SqlxClient {
    pub async fn new_unknown_key(&self, key: UnknownUserKeysFromDb) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"INSERT INTO unknown_user_keys (address, kind) VALUES ($1, $2)"#,
            key.address,
            key.kind
        )
        .execute(&mut tx)
        .await?;

        let (ton_conf, eth_conf, user_address, until_frozen) = match UserAddressType::from_str(
            &key.kind,
        )? {
            UserAddressType::TonPubkey => {
                let (eth_conf, user_address, until_frozen) = sqlx::query!(r#"UPDATE user_keys SET ton_pubkey_is_confirmed = true WHERE ton_pubkey = $1 RETURNING eth_address_is_confirmed, user_address, until_frozen"#, key.address).fetch_one(&mut tx).await.map(|x| (x.eth_address_is_confirmed, x.user_address, x.until_frozen)).unwrap_or((false, "".to_string(), 0));
                (true, eth_conf, user_address, until_frozen)
            }
            UserAddressType::EthAddress => {
                let (ton_conf, user_address, until_frozen) = sqlx::query!(r#"UPDATE user_keys SET eth_address_is_confirmed = true WHERE eth_address = $1 RETURNING ton_pubkey_is_confirmed, user_address, until_frozen"#, key.address).fetch_one(&mut tx).await.map(|x| (x.ton_pubkey_is_confirmed, x.user_address, x.until_frozen)).unwrap_or((false, "".to_string(), 0));
                (ton_conf, true, user_address, until_frozen)
            }
        };

        #[allow(unused_must_use)]
        if ton_conf && eth_conf {
            sqlx::query!(
                r#"UPDATE user_balances SET user_kind = $1, until_frozen = $3 WHERE user_address = $2"#,
                UserType::Relay.to_string(),
                user_address,
                until_frozen,
            ).execute(&mut tx).await;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn is_key_already_confirmed(&self, address: Vec<u8>) -> bool {
        sqlx::query!(
            r#"SELECT * FROM unknown_user_keys WHERE address = $1"#,
            address
        )
        .fetch_one(&self.pool)
        .await
        .is_ok()
    }
}
