use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

#[derive(Debug, Clone, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ProposalCreated {
    #[abi(uint32)]
    pub proposal_id: u32,
    #[abi(address)]
    pub proposer: MsgAddressInt,
    #[abi(array)]
    pub ton_actions: Vec<TonAction>,
    #[abi(array)]
    eth_actions: Vec<EthAction>,
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
