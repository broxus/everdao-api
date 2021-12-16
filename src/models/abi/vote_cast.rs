use nekoton_abi::*;

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain)]
pub struct VoteCast {
    #[abi(uint32)]
    pub proposal_id: u32,
    #[abi(bool)]
    pub support: bool,
    #[abi(uint128)]
    pub votes: u128,
    #[abi(string)]
    pub reason: String,
}
