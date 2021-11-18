use crate::models::sqlx::RawTransactionFromDb;
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn count_raw_transactions(&self) -> i64 {
        sqlx::query!(r#"SELECT COUNT(*) FROM raw_transactions"#)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.count.unwrap_or_default())
            .map_err(|e| {
                log::error!("{}", e);
                e
            })
            .unwrap_or_default()
    }

    pub async fn new_raw_transaction(&self, transaction: RawTransactionFromDb) {
        if let Err(e) = sqlx::query!(
            r#"INSERT INTO raw_transactions (transaction, transaction_hash, timestamp_block, timestamp_lt) VALUES($1, $2, $3, $4) ON CONFLICT DO NOTHING"#,
            transaction.transaction,
            transaction.transaction_hash,
            transaction.timestamp_block,
            transaction.timestamp_lt
        )
        .execute(&self.pool)
        .await {
            log::error!("{}", e);
        }
    }

    pub async fn get_raw_transactions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RawTransactionFromDb>, anyhow::Error> {
        sqlx::query_as!(
            RawTransactionFromDb,
            r#"SELECT * FROM raw_transactions ORDER BY timestamp_block, timestamp_lt LIMIT $1 OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }
}
