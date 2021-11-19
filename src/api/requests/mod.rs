use crate::models::event_type::EventType;
use crate::models::stakeholders_ordering::StakeholdersOrdering;
use crate::models::timeframe::Timeframe;
use crate::models::transaction_ordering::TransactionOrdering;
use crate::models::transfer_status::TransferStatus;
use crate::models::transfers_ordering::TransfersOrdering;
use crate::models::user_type::UserType;
use serde::Deserialize;
use sqlx::types::Decimal;

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search stakeholders")]
pub struct SearchProposalsRequest {
    pub limit: i32,
    pub offset: i32,
    #[opg("user_balance_ge", string)]
    pub user_balance_ge: Option<Decimal>,
    #[opg("user_balance_ge", string)]
    pub user_balance_le: Option<Decimal>,
    pub stakeholder_kind: Option<UserType>,
    #[opg("until_frozen_ge", string)]
    pub until_frozen_ge: Option<Decimal>,
    #[opg("until_frozen_le", string)]
    pub until_frozen_le: Option<Decimal>,
    #[opg("last_reward_ge", string)]
    pub last_reward_ge: Option<Decimal>,
    #[opg("last_reward_le", string)]
    pub last_reward_le: Option<Decimal>,
    #[opg("total_reward_ge", string)]
    pub total_reward_ge: Option<Decimal>,
    #[opg("total_reward_le", string)]
    pub total_reward_le: Option<Decimal>,
    #[opg("frozen_stake_ge", string)]
    pub frozen_stake_ge: Option<Decimal>,
    #[opg("frozen_stake_le", string)]
    pub frozen_stake_le: Option<Decimal>,
    pub created_at_ge: Option<i64>,
    pub created_at_le: Option<i64>,
    pub ordering: Option<StakeholdersOrdering>,
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search transactions")]
pub struct SearchTransactionsRequest {
    pub limit: i32,
    pub offset: i32,
    pub transaction_kind: Option<EventType>,
    #[opg("amount_ge", string)]
    pub amount_ge: Option<Decimal>,
    #[opg("amount_le", string)]
    pub amount_le: Option<Decimal>,
    pub timestamp_block_ge: Option<i64>,
    pub timestamp_block_le: Option<i64>,
    pub ordering: Option<TransactionOrdering>,
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Graph request")]
pub struct GraphRequest {
    pub from: i64,
    pub to: i64,
    pub timeframe: Timeframe,
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("User page staking request")]
pub struct UserPageStakingRequest {
    pub user_address: String,
}

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
