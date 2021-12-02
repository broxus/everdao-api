use rust_decimal::Decimal;
use ton_block::MsgAddressInt;

use crate::models::{VoteOrdering};

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

impl CreateVote {
    pub fn new(
        timestamp_block: i32,
        message_hash: Vec<u8>,
        transaction_hash: Vec<u8>,
        vote: super::abi::VoteCast,
        voter: MsgAddressInt,
    ) -> Self {
        Self {
            proposal_id: vote.proposal_id as i32,
            votes: Decimal::from(vote.votes),
            reason: vote.reason,
            support: vote.support,
            voter: format!(
                "{}:{}",
                voter.workchain_id(),
                voter.address().to_hex_string()
            ),
            message_hash,
            transaction_hash,
            timestamp_block,
        }
    }
}
