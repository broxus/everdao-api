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
    #[abi(name = "eventABI")]
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
    #[abi(name = "chainId")]
    pub chain_id: u32,
    #[abi(name = "eventEmitter", unpack_with = "uint160_bytes::unpack")]
    pub event_emitter: [u8; 20], //uint160
    #[abi(name = "eventBlocksToConfirm")]
    pub event_blocks_to_confirm: u16,
    #[abi(address)]
    pub proxy: MsgAddressInt,
    #[abi(name = "startBlockNumber")]
    pub start_block_number: u32,
    #[abi(name = "endBlockNumber")]
    pub end_block_number: u32,
}

// {
//       "name": "getDetails",
//       "inputs": [
//         {"name":"answerId","type":"uint32"}
//       ],
//       "outputs": [
//         {"components":[{"name":"eventABI","type":"bytes"},{"name":"staking","type":"address"},{"name":"eventInitialBalance","type":"uint64"},{"name":"eventCode","type":"cell"}],"name":"_basicConfiguration","type":"tuple"},
//         {"components":[{"name":"chainId","type":"uint32"},{"name":"eventEmitter","type":"uint160"},{"name":"eventBlocksToConfirm","type":"uint16"},{"name":"proxy","type":"address"},{"name":"startBlockNumber","type":"uint32"},{"name":"endBlockNumber","type":"uint32"}],"name":"_networkConfiguration","type":"tuple"},
//         {"name":"_meta","type":"cell"}
//       ]
//     },
