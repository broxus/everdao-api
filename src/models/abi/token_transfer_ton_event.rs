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
pub struct GetDecodedData {
    #[abi]
    pub wid: i8,
    #[abi(unpack_with = "uint256_bytes::unpack")]
    pub addr: UInt256,
    #[abi]
    pub tokens: u128,
    #[abi(unpack_with = "uint160_bytes::unpack")]
    pub ethereum_address: [u8; 20], //uint160
    #[abi(address)]
    pub owner_address: MsgAddressInt,
    #[abi(name = "chainId")]
    pub chain_id: u32,
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
    #[abi(array, name = "_signatures")]
    pub signatures: Vec<Vec<u8>>,
    #[abi]
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
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct VoteData {
    #[abi(name = "eventTransactionLt")]
    pub event_transaction_lt: u64,
    #[abi(name = "eventTimestamp")]
    pub event_timestamp: u32,
    #[abi(name = "eventData")]
    pub event_data: Cell,
}
