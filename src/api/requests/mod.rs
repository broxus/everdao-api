use rust_decimal::Decimal;
use serde::Deserialize;

use crate::models::{ProposalOrdering, ProposalState};

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search stakeholders")]
pub struct SearchProposalRequest {
    pub transaction_hash: Option<String>,
    pub user_address: Option<String>,
    pub ton_token_address: Option<String>,
    pub eth_token_address: Option<String>,
    pub limit: i32,
    pub offset: i32,
    #[opg("volume_exec_ge", string)]
    pub volume_exec_ge: Option<Decimal>,
    #[opg("volume_exec_le", string)]
    pub volume_exec_le: Option<Decimal>,
    pub chain_id: Option<i32>,
    #[opg("status", string)]
    pub status: Option<TransferStatus>,
    pub updated_at_ge: Option<i64>,
    pub updated_at_le: Option<i64>,
    pub created_at_ge: Option<i64>,
    pub created_at_le: Option<i64>,
    pub ordering: Option<TransfersOrdering>,
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search Votes Request")]
pub struct SearchVotesRequest {
    pub user_address: Option<String>,
    pub ton_token_address: Option<String>,
    pub eth_token_address: Option<String>,
    pub limit: i32,
    pub offset: i32,
    #[opg("volume_exec_ge", string)]
    pub volume_exec_ge: Option<Decimal>,
    #[opg("volume_exec_le", string)]
    pub volume_exec_le: Option<Decimal>,
    pub chain_id: Option<i32>,
    #[opg("status", string)]
    pub status: Option<TransferStatus>,
    pub updated_at_ge: Option<i64>,
    pub updated_at_le: Option<i64>,
    pub created_at_ge: Option<i64>,
    pub created_at_le: Option<i64>,
    pub ordering: Option<TransfersOrdering>,
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search Proposal Votes Request")]
pub struct SearchProposalVotesRequest {
    pub user_address: Option<String>,
    pub ton_token_address: Option<String>,
    pub eth_token_address: Option<String>,
    pub limit: i32,
    pub offset: i32,
    #[opg("volume_exec_ge", string)]
    pub volume_exec_ge: Option<Decimal>,
    #[opg("volume_exec_le", string)]
    pub volume_exec_le: Option<Decimal>,
    pub chain_id: Option<i32>,
    #[opg("status", string)]
    pub status: Option<TransferStatus>,
    pub updated_at_ge: Option<i64>,
    pub updated_at_le: Option<i64>,
    pub created_at_ge: Option<i64>,
    pub created_at_le: Option<i64>,
    pub ordering: Option<TransfersOrdering>,
}
