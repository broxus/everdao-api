use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

#[derive(Debug, Clone, UnpackAbi, KnownParamType)]
pub struct GetDetails {
    #[abi(uint128)]
    pub token_balance: u128,
    #[abi(uint32)]
    pub relay_lock_until: u32,
    #[abi(uint32)]
    pub current_version: u32,
    #[abi(array)]
    pub reward_rounds: Vec<GetDetailsRewardRound>,
    #[abi(with = "uint160_bytes")]
    pub relay_eth_address: [u8; 20],
    #[abi(bool)]
    pub eth_address_confirmed: bool,
    #[abi(with = "uint256_bytes")]
    pub relay_ton_pubkey: UInt256,
    #[abi(bool)]
    pub ton_pubkey_confirmed: bool,
    #[abi(bool)]
    pub slashed: bool,
    #[abi(address)]
    pub root: MsgAddressInt,
    #[abi(address)]
    pub user: MsgAddressInt,
    #[abi(address)]
    pub dao_root: MsgAddressInt,
}

#[derive(Debug, Clone, UnpackAbi, KnownParamType)]
pub struct GetDetailsRewardRound {
    #[abi(uint128)]
    pub reward_balance: u128,
    #[abi(uint128)]
    pub reward_debt: u128,
}
