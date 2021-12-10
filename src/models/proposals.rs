use rust_decimal::Decimal;

use crate::models::*;

#[derive(Debug, serde::Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Search proposals request")]
pub struct SearchProposalsRequest {
    pub limit: i32,
    pub offset: i32,
    pub ordering: Option<ProposalOrdering>,
    pub proposal_id: Option<i32>,
    pub proposer: Option<String>,
    pub start_time_ge: Option<i32>,
    pub start_time_le: Option<i32>,
    pub end_time_ge: Option<i32>,
    pub end_time_le: Option<i32>,
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
    pub for_votes: Decimal,
    pub against_votes: Decimal,
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

impl From<TonAction> for ProposalTonAction {
    fn from(action: TonAction) -> Self {
        Self {
            value: action.value.to_string(),
            target: action.target.to_hex_string(),
            payload: action.payload.to_hex_string(true),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct UpdateProposalVotes {
    pub for_votes: Decimal,
    pub against_votes: Decimal,
}
