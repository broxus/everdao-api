use nekoton_abi::*;
use ton_block::MsgAddressInt;

use crate::models::{EthAction, TonAction};

#[derive(Debug, Clone, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ExpectedProposalAddress {
    #[abi(address)]
    pub value0: MsgAddressInt,
}

#[derive(Debug, Clone, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ProposalCreated {
    #[abi(uint32)]
    pub proposal_id: u32,
    #[abi(address)]
    pub proposer: MsgAddressInt,
    #[abi(array)]
    pub ton_actions: Vec<TonAction>,
    #[abi(array)]
    pub eth_actions: Vec<EthAction>,
    #[abi(string)]
    pub description: String,
}
