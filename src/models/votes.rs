use rust_decimal::Decimal;

use crate::models::VoteOrdering;

#[derive(Debug, serde::Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search votes request")]
pub struct SearchVotesRequest {
    pub limit: i32,
    pub offset: i32,
    pub ordering: Option<VoteOrdering>,
    pub proposal_id: Option<i32>,
    pub voter: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct CreateVote {
    pub proposal_id: i32,
    pub voter: String,
    pub support: bool,
    pub reason: String,
    pub votes: Decimal,
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
}
