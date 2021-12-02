use nekoton_abi::*;
use std::str::FromStr;
use ton_block::MsgAddressInt;
use ton_types::{Cell, UInt256};

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ProposalOverview {
    #[abi(address, name = "proposer_")]
    pub proposer: MsgAddressInt,
    #[abi(string, name = "description_")]
    pub description: String,
    #[abi(uint32, name = "startTime_")]
    pub start_time: u32,
    #[abi(uint32, name = "endTime_")]
    pub end_time: u32,
    #[abi(uint32, name = "executionTime_")]
    pub execution_time: u32,
    #[abi(uint128, name = "forVotes_")]
    pub for_votes: u128,
    #[abi(uint128, name = "againstVotes_")]
    pub against_votes: u128,
    #[abi(uint128, name = "quorumVotes_")]
    pub quorum_votes: u128,
    #[abi(uint8, name = "state_")]
    pub state: u8,
}

#[derive(Debug, Clone, UnpackAbi, KnownParamType)]
pub struct TonAction {
    #[abi(uint128)]
    pub value: u128,
    #[abi(with = "address_only_hash")]
    pub target: UInt256,
    #[abi(cell)]
    pub payload: ton_types::Cell,
}

#[derive(Debug, Clone, UnpackAbi, KnownParamType)]
pub struct EthAction {
    #[abi(with = "uint256_bytes")]
    pub value: UInt256,
    #[abi(uint32)]
    pub chain_id: u32,
    #[abi(with = "uint160_bytes")]
    pub target: [u8; 20],
    #[abi(string)]
    pub signature: String,
    #[abi(bytes)]
    pub call_data: Vec<u8>,
}

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain)]
pub struct GetActions {
    #[abi(array, name = "value0")]
    pub ton_actions: Vec<TonAction>,
    #[abi(array, name = "value1")]
    pub eth_actions: Vec<EthAction>,
}
