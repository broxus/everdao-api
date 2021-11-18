use crate::models::sqlx::{RelayEventFromDb, TransferFromDb};
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn new_relay_event(
        &self,
        relay_event: RelayEventFromDb,
        update_transfer: TransferFromDb,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"UPDATE transfers SET confirm_votes = $1, reject_votes = $2, status = $3,
        timestamp_block_updated_at = $4 WHERE contract_address = $5"#,
            update_transfer.confirm_votes,
            update_transfer.reject_votes,
            update_transfer.status,
            update_transfer.timestamp_block_updated_at,
            update_transfer.contract_address
        )
        .execute(&mut tx)
        .await?;

        sqlx::query!(r#"INSERT INTO relay_events (relay_user_address, contract_address, ton_pub_key, transfer_user_address, 
        status, volume_exec, transfer_kind, currency_address, timestamp_block) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        relay_event.relay_user_address,
        relay_event.contract_address,
        relay_event.ton_pub_key,
        relay_event.transfer_user_address,
        relay_event.status,
        relay_event.volume_exec,
        relay_event.transfer_kind,
        relay_event.currency_address,
        relay_event.timestamp_block).execute(&mut tx).await?;

        tx.commit().await?;
        Ok(())
    }
}
