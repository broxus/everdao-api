use std::convert::TryFrom;

use nekoton::transport::models::RawTransaction;
use rust_decimal::Decimal;
use ton_block::{GetRepresentationHash, Serializable};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RawTransactionFromDb {
    pub transaction: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub timestamp_lt: i64,
    pub created_at: i64,
}

impl TryFrom<RawTransaction> for RawTransactionFromDb {
    type Error = anyhow::Error;

    fn try_from(raw_transaction: RawTransaction) -> Result<Self, Self::Error> {
        let raw_transaction_hash = raw_transaction.data.hash()?.as_slice().to_vec();
        let bytes = raw_transaction.data.write_to_bytes()?;

        Ok(RawTransactionFromDb {
            transaction: bytes,
            transaction_hash: raw_transaction_hash,
            timestamp_block: raw_transaction.data.now as i32,
            timestamp_lt: raw_transaction.data.lt as i64,
            created_at: 0,
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
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub created_at: i64,
}
