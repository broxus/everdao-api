use nekoton_abi::UnpackAbiPlain;
use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::{Cell, UInt256};

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct Confirm {
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub relay: UInt256,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct Reject {
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub relay: UInt256,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct GetDetails {
    #[abi(name = "_eventInitData")]
    pub event_init_data: EventInitData,
    #[abi(name = "_status")]
    pub status: u8,
    #[abi(array, name = "_confirms", unpack_with = "array_uint256_bytes::unpack")]
    pub confirms: Vec<UInt256>,
    #[abi(array, name = "_rejects", unpack_with = "array_uint256_bytes::unpack")]
    pub rejects: Vec<UInt256>,
    #[abi(array, unpack_with = "array_uint256_bytes::unpack")]
    pub empty: Vec<UInt256>,
    #[abi(uint128)]
    pub balance: u128,
    #[abi(name = "_initializer")]
    pub initializer: MsgAddressInt,
    #[abi(name = "_meta")]
    pub meta: Cell,
    #[abi(name = "_requiredVotes")]
    pub required_votes: u32,
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct EventInitData {
    #[abi(name = "voteData")]
    pub vote_data: VoteData,
    #[abi]
    pub configuration: MsgAddressInt,
    #[abi]
    pub staking: MsgAddressInt,
    #[abi(name = "chainId")]
    pub chain_id: u32,
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct VoteData {
    #[abi(name = "eventTransaction", unpack_with = "uint256_bytes::unpack")]
    pub event_transaction: UInt256,
    #[abi(name = "eventIndex")]
    pub event_index: u32,
    #[abi(name = "eventData")]
    pub event_data: Cell,
    #[abi(name = "eventBlockNumber")]
    pub event_block_number: u32,
    #[abi(name = "eventBlock", unpack_with = "uint256_bytes::unpack")]
    pub event_block: UInt256,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct GetDecodedData {
    #[abi]
    pub tokens: u128,
    #[abi]
    pub wid: i8,
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub owner_addr: UInt256,
    #[abi]
    pub owner_address: MsgAddressInt,
}
