use indexer_lib::TransactionExt;
use rust_decimal::Decimal;
use std::convert::TryFrom;
use ton_block::{Serializable, Transaction};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RawTransactionFromDb {
    pub transaction: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub timestamp_lt: i64,
    pub created_at: i64,
}

impl TryFrom<Transaction> for RawTransactionFromDb {
    type Error = anyhow::Error;

    fn try_from(transaction: Transaction) -> Result<Self, Self::Error> {
        let raw_transaction_hash = transaction.tx_hash()?.as_slice().to_vec();
        let bytes = transaction.write_to_bytes()?;

        Ok(RawTransactionFromDb {
            transaction: bytes,
            transaction_hash: raw_transaction_hash,
            timestamp_block: transaction.now as i32,
            timestamp_lt: transaction.lt as i64,
            created_at: 0,
        })
    }

    /*fn from(transaction: Transaction) -> Self {
        RawTransaction {
            hash: transaction.tx_hash().unwrap(),
            data: transaction,
        };
    }*/
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ProposalFromDb {
    pub proposal_id: i32,
    pub contract_address: String,
    pub proposer: String,
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
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
    pub grace_period: i64,
    pub updated_at: i64,
    pub created_at: i64,
    pub canceled_at: Option<i32>,
    pub executed_at: Option<i32>,
    pub queued_at: Option<i32>,
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
