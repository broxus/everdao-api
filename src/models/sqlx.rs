use crate::models::user_type::UserType;
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct RawTransactionsFromDb {
    pub transaction: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub timestamp_lt: i64,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct VoteFromDb {
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub transaction_kind: String,
    pub user_address: String,
    pub user_public_key: Option<String>,
    pub bridge_exec: Decimal,
    pub timestamp_block: i32,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct ProposalFromDb {
    pub user_address: String,
    pub user_kind: String,
    pub stake_balance: Decimal,
    pub frozen_stake: Decimal,
    pub last_reward: Decimal,
    pub total_reward: Decimal,
    pub until_frozen: i32,
    pub updated_at: i64,
    pub created_at: i64,
}

impl Default for ProposalFromDb {
    fn default() -> Self {
        Self {
            user_address: "".to_string(),
            user_kind: UserType::Ordinary.to_string(),
            stake_balance: Default::default(),
            frozen_stake: Default::default(),
            last_reward: Default::default(),
            total_reward: Default::default(),
            until_frozen: 0,
            updated_at: 0,
            created_at: 0,
        }
    }
}
