use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::{ProposalActions, ProposalFromDb, VoteFromDb};

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal response")]
pub struct ProposalResponse {
    pub proposal_id: i32,
    pub contract_address: String,
    pub proposer: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
    pub execution_time: i32,
    #[opg("forVotes", string)]
    pub for_votes: Decimal,
    #[opg("againstVotes", string)]
    pub against_votes: Decimal,
    #[opg("quorumVotes", string)]
    pub quorum_votes: Decimal,
    pub message_hash: String,
    pub transaction_hash: String,
    pub timestamp_block: i32,
    pub actions: ProposalActions,
    pub executed: bool,
    pub canceled: bool,
    pub queued: bool,
    pub grace_period: i32,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Vote response")]
pub struct VoteResponse {
    pub transaction_hash: Option<String>,
    pub contract_address: Option<String>,
    #[opg("volume_exec", string)]
    pub volume_exec: Decimal,
    pub currency_address: String,
    pub transfer_kind: String,
    pub status: String,
    pub required_votes: i32,
    pub confirm_votes: i32,
    pub reject_votes: i32,
    pub chain_id: i32,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Stakeholders table response")]
pub struct ProposalsResponse {
    pub proposals: Vec<ProposalResponse>,
    pub total_count: i32,
}

impl From<(Vec<ProposalFromDb>, i32)> for ProposalsResponse {
    fn from((proposals, total_count): (Vec<ProposalFromDb>, i32)) -> Self {
        Self {
            proposals: proposals
                .into_iter()
                .map(|x| ProposalResponse {
                    proposal_id: x.proposal_id,
                    contract_address: x.contract_address,
                    proposer: x.proposer,
                    description: x.description,
                    start_time: x.start_time,
                    end_time: x.end_time,
                    execution_time: x.execution_time,
                    for_votes: x.for_votes,
                    against_votes: x.against_votes,
                    quorum_votes: x.quorum_votes,
                    message_hash: hex::encode(x.message_hash),
                    transaction_hash: hex::encode(x.transaction_hash),
                    timestamp_block: x.timestamp_block,
                    actions: serde_json::from_value(x.actions).unwrap(),
                    executed: x.executed,
                    canceled: x.canceled,
                    queued: x.queued,
                    grace_period: x.grace_period,
                    updated_at: x.updated_at,
                    created_at: x.created_at,
                })
                .collect::<Vec<_>>(),
            total_count,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Votes response")]
pub struct VotesResponse {
    pub votes: Vec<VoteResponse>,
    pub total_count: i64,
}

impl From<(Vec<VoteFromDb>, i64)> for VotesResponse {
    fn from((votes, total_count): (Vec<VoteFromDb>, i64)) -> Self {
        Self {
            votes: votes
                .into_iter()
                .map(|x| VoteResponse {
                    transaction_hash: x.ton_transaction_hash.map(hex::encode),
                    contract_address: x.contract_address,
                    volume_exec: x.volume_exec,
                    currency_address: x.ton_token_address,
                    transfer_kind: x.transfer_kind,
                    status: x.status,
                    required_votes: x.required_votes,
                    confirm_votes: x.confirm_votes,
                    reject_votes: x.reject_votes,
                    chain_id: x.chain_id,
                    created_at: x.timestamp_block_created_at as i64 * 1000,
                })
                .collect::<Vec<_>>(),
            total_count,
        }
    }
}
