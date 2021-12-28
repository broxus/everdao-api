use std::convert::TryFrom;

use rust_decimal::Decimal;
use ton_block::{GetRepresentationHash, Serializable, Transaction};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, sqlx::Type)]
#[sqlx(type_name = "raw_transaction_state_type", rename_all = "PascalCase")]
pub enum RawTransactionState {
    Idle,
    Fail,
    Success,
    InProgress,
}

impl Default for RawTransactionState {
    fn default() -> RawTransactionState {
        RawTransactionState::InProgress
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct RawTransactionFromDb {
    pub transaction: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub timestamp_lt: i64,
    pub created_at: i64,
    pub state: RawTransactionState,
}

impl TryFrom<Transaction> for RawTransactionFromDb {
    type Error = anyhow::Error;

    fn try_from(transaction: Transaction) -> Result<Self, Self::Error> {
        let bytes = transaction.write_to_bytes()?;
        let transaction_hash = transaction.hash()?.as_slice().to_vec();

        Ok(RawTransactionFromDb {
            transaction: bytes,
            transaction_hash,
            timestamp_block: transaction.now as i32,
            timestamp_lt: transaction.lt as i64,
            ..Default::default()
        })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ProposalFromDb {
    pub id: i32,
    pub address: String,
    pub proposer: String,
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
    pub grace_period: i64,
    pub time_lock: i64,
    pub voting_delay: i64,
    pub for_votes: Decimal,
    pub against_votes: Decimal,
    pub quorum_votes: Decimal,
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub actions: serde_json::Value,
    pub executed: bool,
    pub canceled: bool,
    pub queued: bool,
    pub executed_at: Option<i32>,
    pub canceled_at: Option<i32>,
    pub queued_at: Option<i32>,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VoteFromDb {
    pub proposal_id: i32,
    pub voter: String,
    pub support: bool,
    pub reason: String,
    pub votes: Decimal,
    pub locked: bool,
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub created_at: i64,
}
