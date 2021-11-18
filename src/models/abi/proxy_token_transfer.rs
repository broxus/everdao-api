use nekoton_abi::UnpackAbiPlain;
use nekoton_abi::*;
use ton_block::MsgAddressInt;

#[derive(UnpackAbiPlain, Debug, Clone)]
pub struct GetDetails {
    #[abi]
    pub value0: Value0,
    #[abi(address)]
    pub value1: MsgAddressInt,
    #[abi(uint128)]
    pub value2: u128,
    #[abi(bool)]
    pub value3: bool,
}

#[derive(UnpackAbiPlain, UnpackAbi, Debug, Clone)]
pub struct Value0 {
    #[abi(address, name = "tonConfiguration")]
    pub ton_configuration: MsgAddressInt,
    #[abi(array, name = "ethereumConfigurations")]
    pub ethereum_configurations: Vec<MsgAddressInt>,
    #[abi(array, name = "outdatedTokenRoots")]
    pub outdated_token_roots: Vec<MsgAddressInt>,
    #[abi(address, name = "tokenRoot")]
    pub token_root: MsgAddressInt,
    #[abi(name = "settingsDeployWalletGrams", uint128)]
    pub settings_deploy_wallet_grams: u128,
}
