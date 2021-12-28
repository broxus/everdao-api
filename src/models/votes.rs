use rust_decimal::Decimal;
use serde::Deserialize;

use crate::models::*;
use crate::utils::*;

pub type VotesSearch = Paginated<Ordered<VoteFilters, VotesOrdering>>;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct VoteFilters {
    pub voter: Option<String>,
    pub proposal_id: Option<u32>,
    pub support: Option<bool>,
    pub locked: Option<bool>,
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Hash)]
pub struct UnlockVote {
    pub proposal_id: i32,
    pub voter: String,
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[opg("Votes ordering")]
pub struct VotesOrdering {
    pub column: VoteColumn,
    pub direction: Direction,
}

impl Default for VotesOrdering {
    fn default() -> Self {
        Self {
            column: VoteColumn::CreatedAt,
            direction: Direction::Descending,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Vote column")]
pub enum VoteColumn {
    CreatedAt,
}
