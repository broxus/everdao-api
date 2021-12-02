use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::{ProposalActions, ProposalFromDb, ProposalState, VoteFromDb};

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal response")]
pub struct ProposalResponse {
    pub proposal_id: i32,
    pub contract_address: String,
    pub proposer: String,
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
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
    pub grace_period: i64,
    pub state: ProposalState,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Vote response")]
pub struct VoteResponse {
    pub proposal_id: i32,
    pub voter: String,
    pub support: bool,
    pub reason: String,
    #[opg("votes", string)]
    pub votes: Decimal,
    pub message_hash: String,
    pub transaction_hash: String,
    pub timestamp_block: i32,
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
        let now = Utc::now().timestamp();
        Self {
            proposals: proposals
                .into_iter()
                .map(|x| {

                    let state = if x.canceled {
                         ProposalState.Canceled
                    } else if x.executed {
                         ProposalState.Executed
                    } else if now <= x.start_time {
                         ProposalState.Pending
                    } else if now <= x.end_time {
                         ProposalState.Active
                    } else if x.for_votes <= x.against_votes || x.for_votes < x.quorum_votes {
                         ProposalState.Failed
                    } else if x.execution_time == 0 {
                         ProposalState.Succeeded
                    } else if now > x.execution_time + config.x.grace_period {
                         ProposalState.Expired
                    } else {
                         ProposalState.Queued
                    };

                    ProposalResponse {
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
                        state,
                        updated_at: x.updated_at,
                        created_at: x.created_at,
                    }
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
                    transaction_hash: hex::encode(x.transaction_hash),
                    message_hash: hex::encode(x.message_hash),
                    proposal_id: x.proposal_id,
                    voter: x.voter,
                    support: x.support,
                    reason: x.reason,
                    votes: x.votes,
                    timestamp_block: x.timestamp_block,
                    created_at: x.created_at,
                })
                .collect::<Vec<_>>(),
            total_count,
        }
    }
}
