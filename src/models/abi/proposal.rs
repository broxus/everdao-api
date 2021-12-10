use nekoton_abi::*;
use ton_block::MsgAddressInt;

#[derive(Debug, Clone, PackAbi, UnpackAbi, KnownParamType)]
pub struct ProposalConfig {
    #[abi(uint32)]
    pub voting_delay: u32,
    #[abi(uint32)]
    pub voting_period: u32,
    #[abi(uint128)]
    pub quorum_votes: u128,
    #[abi(uint32)]
    pub time_lock: u32,
    #[abi(uint128)]
    pub threshold: u128,
    #[abi(uint32)]
    pub grace_period: u32,
}

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ProposalOverview {
    #[abi(address)]
    pub proposer: MsgAddressInt,
    #[abi(string)]
    pub description: String,
    #[abi(uint32)]
    pub start_time: u32,
    #[abi(uint32)]
    pub end_time: u32,
    #[abi(uint32)]
    pub execution_time: u32,
    #[abi(uint128)]
    pub for_votes: u128,
    #[abi(uint128)]
    pub against_votes: u128,
    #[abi(uint128)]
    pub quorum_votes: u128,
    #[abi(uint8)]
    pub state: u8,
}
