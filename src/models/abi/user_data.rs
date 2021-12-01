use nekoton_abi::UnpackAbiPlain;
use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct RelayKeysUpdated {
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub ton_pubkey: UInt256,
    #[abi(unpack_with = "uint160_bytes::unpack")]
    pub eth_address: [u8; 20], //uint160
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct TonPubkeyConfirmed {
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub ton_pubkey: UInt256,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct EthAddressConfirmed {
    #[abi(unpack_with = "uint160_bytes::unpack")]
    pub eth_addr: [u8; 20], //uint160
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct RelayMembershipRequested {
    #[abi(uint32)]
    pub round_num: u32,
    #[abi(uint128)]
    pub tokens: u128,
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub ton_pubkey: UInt256,
    #[abi(unpack_with = "uint160_bytes::unpack")]
    pub eth_address: [u8; 20], //uint160
    #[abi(uint32)]
    pub lock_until: u32,
}

#[derive(UnpackAbiPlain, Debug, Clone, KnownParamTypePlain)]
pub struct GetDetails {
    #[abi]
    pub value0: GetDetailsValue0,
}

#[derive(Debug, Clone, Default, UnpackAbiPlain, KnownParamTypePlain)]
pub struct GetDetailsValue0 {
    #[abi(uint128)]
    pub token_balance: u128,
    #[abi(uint32)]
    pub relay_lock_until: u32,
    #[abi(array, name = "rewardRounds")]
    pub reward_rounds: Vec<GetDetailsRewardRound>,
    #[abi(unpack_with = "uint160_bytes::unpack")]
    pub relay_eth_address: [u8; 20], //uint160
    #[abi(bool)]
    pub eth_address_confirmed: bool,
    #[abi(unpack_with = "uint256_bytes::unpack")]
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

#[derive(UnpackAbi, Debug, Clone)]
pub struct GetDetailsRewardRound {
    #[abi(uint128)]
    pub reward_balance: u128,
    #[abi(uint128)]
    pub reward_debt: u128,
}
