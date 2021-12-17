use chrono::Utc;
use nekoton_utils::TrustMe;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::{ProposalActions, ProposalFromDb, ProposalState, VoteFromDb};

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal response")]
pub struct ProposalResponse {
    pub proposal_id: i32,
    pub proposal_address: String,
    pub proposer: String,
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: Option<i64>,
    pub grace_period: i64,
    pub time_lock: i64,
    pub voting_delay: i64,
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
    pub executed_at: Option<i32>,
    pub canceled_at: Option<i32>,
    pub queued_at: Option<i32>,
    pub updated_at: i64,
    pub created_at: i64,
    pub state: ProposalState,
}

impl From<ProposalFromDb> for ProposalResponse {
    fn from(x: ProposalFromDb) -> Self {
        let execution_time = match x.execution_time {
            0 => None,
            _ => Some(x.execution_time),
        };

        let now = Utc::now().timestamp();
        let state = if x.canceled {
            ProposalState::Canceled
        } else if x.executed {
            ProposalState::Executed
        } else if now <= x.start_time {
            ProposalState::Pending
        } else if now <= x.end_time {
            ProposalState::Active
        } else if x.for_votes <= x.against_votes || x.for_votes < x.quorum_votes {
            ProposalState::Failed
        } else if x.execution_time == 0 {
            ProposalState::Succeeded
        } else if now > x.execution_time + x.grace_period {
            ProposalState::Expired
        } else {
            ProposalState::Queued
        };

        Self {
            proposal_id: x.id,
            proposal_address: x.address,
            proposer: x.proposer,
            description: x.description,
            start_time: x.start_time,
            end_time: x.end_time,
            execution_time,
            grace_period: x.grace_period,
            time_lock: x.time_lock,
            voting_delay: x.voting_delay,
            for_votes: x.for_votes,
            against_votes: x.against_votes,
            quorum_votes: x.quorum_votes,
            message_hash: hex::encode(x.message_hash),
            transaction_hash: hex::encode(x.transaction_hash),
            timestamp_block: x.timestamp_block,
            actions: serde_json::from_value(x.actions).trust_me(),
            executed: x.executed,
            canceled: x.canceled,
            queued: x.queued,
            executed_at: x.executed_at,
            canceled_at: x.canceled_at,
            queued_at: x.queued_at,
            updated_at: x.updated_at,
            created_at: x.created_at,
            state,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal table response")]
pub struct ProposalsResponse {
    pub proposals: Vec<ProposalResponse>,
    pub total_count: i64,
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

impl From<VoteFromDb> for VoteResponse {
    fn from(x: VoteFromDb) -> Self {
        Self {
            transaction_hash: hex::encode(x.transaction_hash),
            message_hash: hex::encode(x.message_hash),
            proposal_id: x.proposal_id,
            voter: x.voter,
            support: x.support,
            reason: x.reason,
            votes: x.votes,
            timestamp_block: x.timestamp_block,
            created_at: x.created_at,
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

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal with Vote response")]
pub struct ProposalWithVoteResponse {
    pub vote: VoteResponse,
    pub proposal: ProposalResponse,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposals with Votes response")]
pub struct ProposalsWithVotesResponse {
    pub proposal_with_votes: Vec<ProposalWithVoteResponse>,
    pub total_count: i64,
}
