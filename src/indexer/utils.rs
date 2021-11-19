use crate::models::raw_transactions::create_raw_transaction_from_db;
use crate::models::sqlx::RawTransactionsFromDb;
use crate::sqlx_client::SqlxClient;
use chrono::Utc;
use indexer_lib::TransactionExt;
use nekoton::transport::models::RawTransaction;
use tokio::time::{sleep, Duration};
use ton_block::Transaction;

pub const BRIDGE_SCALE: u32 = 9;
pub const HOUR_SEC: u64 = 3600;
pub const STAKING_CONTRACT_ADDRESS: &str =
    "0:7727ca13859ee381892ee6a0435165d36053188900550cdb02b93ea6bc81c075";

pub async fn update_frozen_staking(sqlx_client: SqlxClient) {
    loop {
        let timestamp_now = Utc::now().timestamp() as i32;
        if let Err(e) = sqlx_client.update_frozen_staking(timestamp_now).await {
            log::error!("update_frozen_staking failed: {}", e);
        }
        sleep(Duration::new(HOUR_SEC, 0)).await;
    }
}

pub fn build_answer_id_camel() -> ton_abi::Token {
    ton_abi::Token::new(
        "answerId",
        ton_abi::TokenValue::Uint(ton_abi::Uint::new(1337, 32)),
    )
}

pub fn from_transaction_to_raws_transactions(
    transaction: Transaction,
) -> (RawTransaction, RawTransactionsFromDb) {
    let raw_transaction = RawTransaction {
        hash: transaction.tx_hash().unwrap(),
        data: transaction,
    };
    let raw_transaction_from_db = create_raw_transaction_from_db(raw_transaction.clone()).unwrap();
    (raw_transaction, raw_transaction_from_db)
}
