use nekoton_abi::UnpackAbiPlain;
use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_types::UInt256;

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct Deposit {
    #[abi(address)]
    pub user: MsgAddressInt,
    #[abi(uint128)]
    pub amount: u128,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct Withdraw {
    #[abi(address)]
    pub user: MsgAddressInt,
    #[abi(uint128)]
    pub amount: u128,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct RewardClaimed {
    #[abi(address)]
    pub user: MsgAddressInt,
    #[abi(uint128)]
    pub reward_tokens: u128,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct NewRewardRound {
    #[abi(uint32)]
    pub round_num: u32,
}

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct GetDetails {
    #[abi(address)]
    pub bridge_event_config_eth_ton: MsgAddressInt,
    #[abi(address)]
    pub bridge_event_config_ton_eth: MsgAddressInt,
    #[abi(name = "tokenRoot", address)]
    pub token_root: MsgAddressInt,
    #[abi(name = "tokenWallet", address)]
    pub token_wallet: MsgAddressInt,
    #[abi(address)]
    pub admin: MsgAddressInt,
    #[abi(address)]
    pub rescuer: MsgAddressInt,
    #[abi(address)]
    pub rewarder: MsgAddressInt,
    #[abi(name = "tokenBalance", uint128)]
    pub token_balance: u128,
    #[abi(name = "rewardTokenBalance", uint128)]
    pub reward_token_balance: u128,
    #[abi(name = "lastRewardTime", uint32)]
    pub last_reward_time: u32,
    #[abi(array, name = "rewardRounds")]
    pub reward_rounds: Vec<RewardRound>,
    #[abi(bool)]
    pub emergency: bool,
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct RewardRound {
    #[abi(name = "accRewardPerShare", unpack_with = "uint256_bytes::unpack")]
    pub acc_reward_per_share: UInt256,
    #[abi(name = "rewardTokens", uint128)]
    pub reward_tokens: u128,
    #[abi(name = "totalReward", uint128)]
    pub total_reward: u128,
    #[abi(name = "startTime", uint32)]
    pub start_time: u32,
}
