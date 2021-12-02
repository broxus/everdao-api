use rust_decimal::Decimal;

use crate::models::{ProposalOrdering, ProposalState};

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
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
    pub proposal_id: i32,
    pub proposer: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
    pub execution_time: i32,
    pub for_votes: Decimal,
    pub against_votes: Decimal,
    pub quorum_votes: Decimal,
    pub message_hash: Vec<u8>,
    pub transaction_hash: Vec<u8>,
    pub timestamp_block: i32,
    pub actions: ProposalActions,
    pub grace_period: i32,
}

impl CreateProposal {
    pub fn new(
        timestamp_block: i32,
        message_hash: Vec<u8>,
        transaction_hash: Vec<u8>,
        proposal_id: u32,
        proposal: super::abi::ProposalOverview,
        eth_actions: Vec<super::abi::EthAction>,
        ton_actions: Vec<super::abi::TonAction>,
        grace_period: u32,
    ) -> Self {
        Self {
            proposal_id: proposal_id as i32,
            proposer: format!(
                "{}:{}",
                proposal.proposer.workchain_id(),
                proposal.proposer.address().to_hex_string()
            ),
            description: proposal.description,
            start_time: proposal.start_time as i32,
            end_time: proposal.end_time as i32,
            execution_time: proposal.execution_time as i32,
            for_votes: Decimal::from(proposal.for_votes),
            against_votes: Decimal::from(proposal.against_votes),
            quorum_votes: Decimal::from(proposal.quorum_votes),
            message_hash,
            transaction_hash,
            timestamp_block,
            actions: ProposalActions {
                ton_actions: ton_actions.into_iter().map(From::from).collect(),
                eth_actions: eth_actions.into_iter().map(From::from).collect(),
            },
            grace_period: grace_period as i32,
        }
    }
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

impl From<super::abi::EthAction> for ProposalEthAction {
    fn from(a: super::abi::EthAction) -> Self {
        Self {
            value: a.value.to_hex_string(),
            chain_id: a.chain_id,
            target: a.target.to_hex(),
            signature: a.signature,
            call_data: a.call_data.to_hex(),
        }
    }
}

impl From<super::abi::TonAction> for ProposalTonAction {
    fn from(a: super::abi::TonAction) -> Self {
        Self {
            value: a.value.to_string(),
            target: a.target.to_hex_string(),
            payload: a.payload.to_hex_string(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdateProposal {
    pub for_votes: Option<Decimal>,
    pub against_votes: Option<Decimal>,
    pub quorum_votes: Option<Decimal>,
    pub executed: Option<bool>,
    pub canceled: Option<bool>,
    pub queued: Option<bool>,
}
