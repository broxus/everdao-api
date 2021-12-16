use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::{Cell, UInt256};

#[derive(Debug, Clone, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ProposalCreated {
    #[abi(uint32, name = "proposalId")]
    pub proposal_id: u32,
    #[abi(address)]
    pub proposer: MsgAddressInt,
    #[abi(array, name = "tonActions")]
    pub ton_actions: Vec<TonAction>,
    #[abi(array, name = "ethActions")]
    pub eth_actions: Vec<EthAction>,
    #[abi(string)]
    pub description: String,
}

#[derive(Debug, Clone, UnpackAbi, KnownParamType)]
pub struct TonAction {
    #[abi(uint128)]
    pub value: u128,
    #[abi(with = "address_only_hash")]
    pub target: UInt256,
    #[abi(cell)]
    pub payload: Cell,
}

#[derive(Debug, Clone, UnpackAbi, KnownParamType)]
pub struct EthAction {
    #[abi(with = "uint256_bytes")]
    pub value: UInt256,
    #[abi(uint32, name = "chainId")]
    pub chain_id: u32,
    #[abi(with = "uint160_bytes")]
    pub target: [u8; 20],
    #[abi(string)]
    pub signature: String,
    #[abi(bytes, name = "callData")]
    pub call_data: Vec<u8>,
}
