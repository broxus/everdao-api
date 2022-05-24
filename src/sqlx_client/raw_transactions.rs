use crate::models::*;
use crate::sqlx_client::*;

impl SqlxClient {
    pub async fn create_raw_transaction(
        &self,
        transaction: RawTransactionFromDb,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO raw_transactions_service (transaction, transaction_hash, timestamp_block, timestamp_lt) VALUES($1, $2, $3, $4) ON CONFLICT DO NOTHING"#,
            transaction.transaction,
            transaction.transaction_hash,
            transaction.timestamp_block,
            transaction.timestamp_lt
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_raw_transactions_by_state(
        &self,
        state: RawTransactionState,
    ) -> Result<Vec<RawTransactionFromDb>, anyhow::Error> {
        sqlx::query_as!(
            RawTransactionFromDb,
            r#"
            SELECT transaction, transaction_hash, timestamp_block, timestamp_lt, created_at, state as "state: _"
            FROM raw_transactions_service
            WHERE state = $1
            ORDER BY timestamp_block"#,
            state as RawTransactionState,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn update_raw_transactions(
        &self,
        transaction_hash: &[u8],
        state: RawTransactionState,
    ) -> Result<RawTransactionFromDb, anyhow::Error> {
        sqlx::query_as!(
            RawTransactionFromDb,
            r#"
            UPDATE raw_transactions_service SET state = $1
            WHERE transaction_hash = $2
            RETURNING transaction,
                transaction_hash,
                timestamp_block,
                timestamp_lt,
                created_at,
                state as "state: _""#,
            state as RawTransactionState,
            transaction_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
    }
}
