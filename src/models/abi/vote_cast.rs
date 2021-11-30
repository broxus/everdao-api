use nekoton_abi::*;

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain)]
pub struct VoteCast {
    #[abi(uint32, name = "proposal_id")]
    pub proposal_id: u32,
    #[abi(bool, name = "support")]
    pub support: bool,
    #[abi(uint128, name = "votes")]
    pub votes: u128,
    #[abi(string, name = "reason")]
    pub reason: String,
}
