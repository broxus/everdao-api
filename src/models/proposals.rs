use std::convert::TryFrom;
use indexer_lib::TransactionExt;
use rust_decimal::Decimal;
use ton_block::{Serializable, Transaction};

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
pub struct CreateProposal {
    pub proposal_id: i32,
    pub proposer: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
    pub execution_time: i32,
    pub for_votes: Decimal,
    pub against_votes: Decimal,
    pub quorum_votes: Decimal,
    pub state: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdateProposal {
    pub for_votes: Option<Decimal>,
    pub against_votes: Option<Decimal>,
    pub quorum_votes: Option<Decimal>,
    pub state: Option<String>,
}

