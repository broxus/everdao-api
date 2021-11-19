use crate::models::user_type::UserType;
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct RawTransactionFromDb {
    pub transaction: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub timestamp_lt: i64,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct TransactionFromDb {
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
pub struct BridgeBalanceFromDb {
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub transaction_kind: String,
    pub user_address: String,
    pub user_balance: Decimal,
    pub reward: Option<Decimal>,
    pub bridge_balance: Decimal,
    pub stakeholders: i32,
    pub average_apr: Decimal,
    pub bridge_reward: Decimal,
    pub timestamp_block: i32,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct UserKeysFromDb {
    pub user_address: String,
    pub ton_pubkey: Vec<u8>,
    pub ton_pubkey_is_confirmed: bool,
    pub eth_address: Vec<u8>,
    pub eth_address_is_confirmed: bool,
    pub until_frozen: i32,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct UnknownUserKeysFromDb {
    pub address: Vec<u8>,
    pub kind: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct GraphDataFromDb {
    pub kind: String,
    pub balance: Decimal,
    pub apr: Decimal,
    pub reward: Decimal,
    pub timestamp: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor)]
pub struct RewardRoundInfoFromDb {
    pub num_round: i32,
    pub start_time: i32,
    pub end_time: i32,
    pub reward_tokens: Decimal,
    pub total_reward: Decimal,
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor, Default)]
pub struct TransferFromDb {
    pub ton_message_hash: Option<Vec<u8>>,
    pub ton_transaction_hash: Option<Vec<u8>>,
    pub contract_address: Option<String>,
    pub event_index: Option<i32>,
    pub eth_transaction_hash: Option<String>,
    pub user_address: String,
    pub volume_exec: Decimal,
    pub ton_token_address: String,
    pub eth_token_address: String,
    pub transfer_kind: String,
    pub status: String,
    pub required_votes: i32,
    pub confirm_votes: i32,
    pub reject_votes: i32,
    pub burn_callback_timestamp_lt: Option<i64>,
    pub timestamp_block_updated_at: i32,
    pub timestamp_block_created_at: i32,
    pub graphql_timestamp: Option<i32>,
    pub chain_id: i32,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, derive_more::Constructor, Default)]
pub struct RelayEventFromDb {
    pub relay_user_address: String,
    pub contract_address: String,
    pub ton_pub_key: Vec<u8>,
    pub transfer_user_address: String,
    pub status: String,
    pub volume_exec: Decimal,
    pub transfer_kind: String,
    pub currency_address: String,
    pub timestamp_block: i32,
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    derive_more::Constructor,
    Default,
    Eq,
    PartialEq,
    Hash,
)]
pub struct VaultInfoFromDb {
    pub vault_address: String,
    pub ton_token_address: String,
    pub eth_token_address: String,
    pub ton_currency_scale: i32,
    pub ton_currency: String,
    pub chain_id: i32,
    pub proxy: String,
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    derive_more::Constructor,
    Default,
    Eq,
    PartialEq,
    Hash,
)]
pub struct GraphqlEndPoint {
    pub chain_id: i32,
    pub url: String,
}
