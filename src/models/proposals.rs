use rust_decimal::Decimal;
use serde::Deserialize;

use crate::models::*;
use crate::utils::*;

pub type ProposalsSearch = Paginated<Ordered<ProposalFilters, ProposalsOrdering>>;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct ProposalFilters {
    pub start_time_ge: Option<u32>,
    pub start_time_le: Option<u32>,

    pub end_time_ge: Option<u32>,
    pub end_time_le: Option<u32>,

    pub proposal_id: Option<u32>,

    pub proposer: Option<String>,

    pub proposal_address: Option<String>,

    pub state: Option<ProposalState>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct CreateProposal {
    pub id: i32,
    pub address: String,
    pub proposer: String,
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub execution_time: i64,
    pub grace_period: i64,
    pub time_lock: i64,
    pub voting_delay: i64,
    pub quorum_votes: Decimal,
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub actions: ProposalActions,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct ProposalActions {
    pub ton_actions: Vec<ProposalTonAction>,
    pub eth_actions: Vec<ProposalEthAction>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct ProposalTonAction {
    pub value: String,
    pub target: String,
    pub payload: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct ProposalEthAction {
    pub value: String,
    pub chain_id: u32,
    pub target: String,
    pub signature: String,
    pub call_data: String,
}

impl From<EthAction> for ProposalEthAction {
    fn from(action: EthAction) -> Self {
        Self {
            value: action.value.to_hex_string(),
            chain_id: action.chain_id,
            target: hex::encode(action.target),
            signature: action.signature,
            call_data: hex::encode(action.call_data),
        }
    }
}

impl TryFrom<TonAction> for ProposalTonAction {
    type Error = anyhow::Error;

    fn try_from(action: TonAction) -> Result<Self, Self::Error> {
        Ok(Self {
            value: action.value.to_string(),
            target: action.target.to_string(),
            payload: base64::encode(ton_types::serialize_toc(&action.payload)?),
        })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct UpdateProposalVotes {
    pub for_votes: Decimal,
    pub against_votes: Decimal,
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[opg("Proposals ordering")]
pub struct ProposalsOrdering {
    pub column: ProposalColumn,
    pub direction: Direction,
}

impl Default for ProposalsOrdering {
    fn default() -> Self {
        Self {
            column: ProposalColumn::CreatedAt,
            direction: Direction::Descending,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal column")]
pub enum ProposalColumn {
    CreatedAt,
}
