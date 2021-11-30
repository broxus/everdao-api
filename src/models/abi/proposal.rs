use nekoton_abi::*;
use ton_block::MsgAddressInt;

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


enum ProposalState {
    Pending,
    Active,
    Canceled,
    Failed,
    Succeeded,
    Expired,
    Queued,
    Executed
}