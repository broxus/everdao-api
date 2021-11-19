pub use crate::models::sqlx::RawTransactionsFromDb;
use anyhow::Context;
use indexer_lib::TransactionExt;
use nekoton::transport::models::RawTransaction;
use ton_block::Serializable;

pub fn create_raw_transaction_from_db(
    raw_transaction: RawTransaction,
) -> Result<RawTransactionsFromDb, anyhow::Error> {
    let raw_transaction_hash = raw_transaction.data.tx_hash()?.as_slice().to_vec();
    let bytes = raw_transaction
        .data
        .write_to_bytes()
        .context("Failed serializing tx to bytes")?;

    Ok(RawTransactionsFromDb {
        transaction: bytes,
        transaction_hash: raw_transaction_hash,
        timestamp_block: raw_transaction.data.now as i32,
        timestamp_lt: raw_transaction.data.lt as i64,
        created_at: 0,
    })
}
