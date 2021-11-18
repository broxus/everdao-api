use crate::models::sqlx::{TransactionFromDb, TransferFromDb, UserBalanceFromDb};
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Stakeholder response")]
pub struct StakeholderResponse {
    pub user_address: String,
    pub user_type: String,
    #[opg("stake_balance", string)]
    pub stake_balance: Decimal,
    #[opg("frozen_stake_balance", string)]
    pub frozen_stake_balance: Decimal,
    #[opg("last_reward", string)]
    pub last_reward: Decimal,
    #[opg("total_reward", string)]
    pub total_reward: Decimal,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Stakeholder response")]
pub struct TransferResponse {
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
pub struct StakeholdersTableResponse {
    pub stakeholders: Vec<StakeholderResponse>,
    pub total_count: i32,
}

impl From<(Vec<UserBalanceFromDb>, i32)> for StakeholdersTableResponse {
    fn from((user_balances, total_count): (Vec<UserBalanceFromDb>, i32)) -> Self {
        Self {
            stakeholders: user_balances
                .into_iter()
                .map(|x| StakeholderResponse {
                    user_address: x.user_address,
                    user_type: x.user_kind,
                    stake_balance: x.stake_balance,
                    frozen_stake_balance: x.frozen_stake,
                    last_reward: x.last_reward,
                    total_reward: x.total_reward,
                    created_at: x.created_at,
                })
                .collect::<Vec<_>>(),
            total_count,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Stakeholders table response")]
pub struct TransfersTableResponse {
    pub transfers: Vec<TransferResponse>,
    pub total_count: i64,
}

impl From<(Vec<TransferFromDb>, i64)> for TransfersTableResponse {
    fn from((transfers, total_count): (Vec<TransferFromDb>, i64)) -> Self {
        Self {
            transfers: transfers
                .into_iter()
                .map(|x| TransferResponse {
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

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Transaction response")]
pub struct TransactionResponse {
    pub transaction_hash: String,
    pub transaction_kind: String,
    #[opg("amount_exec", string)]
    pub amount_exec: Decimal,
    pub timestamp_block: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Transactions table response")]
pub struct TransactionsTableResponse {
    pub transactions: Vec<TransactionResponse>,
    pub total_count: i32,
}

impl From<(Vec<TransactionFromDb>, i32)> for TransactionsTableResponse {
    fn from((transactions, total_count): (Vec<TransactionFromDb>, i32)) -> Self {
        Self {
            transactions: transactions
                .into_iter()
                .map(|x| TransactionResponse {
                    transaction_hash: hex::encode(x.transaction_hash),
                    transaction_kind: x.transaction_kind,
                    amount_exec: x.bridge_exec,
                    timestamp_block: x.timestamp_block as i64 * 1000,
                })
                .collect::<Vec<_>>(),
            total_count,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Graph data response")]
pub struct GraphDataResponse {
    #[opg("data", string)]
    pub data: Decimal,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Main page staking response")]
pub struct MainPageStakingResponse {
    #[opg("tvl", string)]
    pub tvl: Decimal,
    #[opg("tvl_change", string)]
    pub tvl_change: Decimal,
    #[opg("reward_30d", string)]
    pub reward_30d: Decimal,
    #[opg("reward_30d_change", string)]
    pub reward_30d_change: Decimal,
    #[opg("average_apr", string)]
    pub average_apr: Decimal,
    pub stakeholders: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("User page staking response")]
pub struct UserPageStakingResponse {
    #[opg("user_tvl", string)]
    pub user_tvl: Decimal,
    #[opg("user_tvl_change", string)]
    pub user_tvl_change: Decimal,
    #[opg("user_frozen_stake", string)]
    pub user_frozen_stake: Decimal,
    #[opg("user_30d_reward", string)]
    pub user_30d_reward: Decimal,
    #[opg("user_30d_reward_change", string)]
    pub user_30d_reward_change: Decimal,
    #[opg("average_apr", string)]
    pub average_apr: Decimal,
}
