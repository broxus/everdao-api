use rust_decimal::Decimal;

use crate::models::{VoteOrdering, VoteState};

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search votes request")]
pub struct SearchVotesRequest {
    pub limit: i32,
    pub offset: i32,
    pub ordering: Option<VoteOrdering>,
    pub vote_id: Option<i32>,
    pub proposer: Option<String>,
    pub start_time_ge: Option<i32>,
    pub start_time_le: Option<i32>,
    pub end_time_ge: Option<i32>,
    pub end_time_le: Option<i32>,
    pub state: Option<VoteState>,
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
        proposal_id: u32,
        vote: super::abi::VoteOverview,
        eth_actions: Vec<super::abi::EthAction>,
        ton_actions: Vec<super::abi::TonAction>,
        grace_period: u32,
    ) -> Self {
        Self {
            vote_id: vote_id as i32,
            proposer: format!(
                "{}:{}",
                vote.proposer.workchain_id(),
                vote.proposer.address().to_hex_string()
            ),
            description: vote.description,
            start_time: vote.start_time as i64,
            end_time: vote.end_time as i64,
            execution_time: vote.execution_time as i64,
            for_votes: Decimal::from(vote.for_votes),
            against_votes: Decimal::from(vote.against_votes),
            quorum_votes: Decimal::from(vote.quorum_votes),
            message_hash,
            transaction_hash,
            timestamp_block,
            actions: VoteActions {
                ton_actions: ton_actions.into_iter().map(From::from).collect(),
                eth_actions: eth_actions.into_iter().map(From::from).collect(),
            },
            grace_period: grace_period as i64,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct VoteActions {
    pub ton_actions: Vec<VoteTonAction>,
    pub eth_actions: Vec<VoteEthAction>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct VoteTonAction {
    pub value: String,
    pub target: String,
    pub payload: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct VoteEthAction {
    pub value: String,
    pub chain_id: u32,
    pub target: String,
    pub signature: String,
    pub call_data: String,
}

impl From<super::abi::EthAction> for VoteEthAction {
    fn from(a: super::abi::EthAction) -> Self {
        Self {
            value: a.value.to_hex_string(),
            chain_id: a.chain_id,
            target: a.target.to_hex(),
            signature: a.signature,
            call_data: a.call_data.to_hex(),
        }
    }
}

impl From<super::abi::TonAction> for VoteTonAction {
    fn from(a: super::abi::TonAction) -> Self {
        Self {
            value: a.value.to_string(),
            target: a.target.to_hex_string(),
            payload: a.payload.to_hex_string(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdateVote {
    pub for_votes: Option<Decimal>,
    pub against_votes: Option<Decimal>,
    pub quorum_votes: Option<Decimal>,
    pub executed: Option<bool>,
    pub canceled: Option<bool>,
    pub queued: Option<bool>,
}
