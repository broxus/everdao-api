use nekoton_abi::UnpackAbi;
use nekoton_abi::UnpackAbiPlain;
use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::Cell;

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct NewEventContract {
    #[abi(name = "eventContract", address)]
    pub event_contract: MsgAddressInt,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct GetDetails {
    #[abi(name = "_basicConfiguration")]
    pub basic_configuration: BasicConfiguration,
    #[abi(name = "_networkConfiguration")]
    pub network_configuration: NetworkConfiguration,
    #[abi]
    pub _meta: Cell,
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct BasicConfiguration {
    #[abi(name = "eventABI", array)]
    pub event_abi: Vec<u8>,
    #[abi(address)]
    pub staking: MsgAddressInt,
    #[abi(name = "eventInitialBalance")]
    pub event_initial_balance: u64,
    #[abi(name = "eventCode")]
    pub event_code: Cell,
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct NetworkConfiguration {
    #[abi(address, name = "eventEmitter")]
    pub event_emitter: MsgAddressInt,
    #[abi(unpack_with = "uint160_bytes::unpack")]
    pub proxy: [u8; 20], //uint160
    #[abi(name = "startTimestamp")]
    pub start_timestamp: u32,
    #[abi(name = "endTimestamp")]
    pub end_timestamp: u32,
}
