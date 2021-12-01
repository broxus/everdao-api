use nekoton_abi::*;
use ton_block::MsgAddressInt;

#[derive(Debug, Clone, PackAbiPlain, UnpackAbiPlain, KnownParamTypePlain)]
pub struct ProposalOverview {
    #[abi(address, name = "proposer_")]
    pub proposer: MsgAddressInt,
    #[abi(string, name = "description_")]
    pub description: String,
    #[abi(uint32, name = "startTime_")]
    pub start_time: u32,
    #[abi(uint32, name = "endTime_")]
    pub end_time: u32,
    #[abi(uint32, name = "executionTime_")]
    pub execution_time: u32,
    #[abi(uint128, name = "forVotes_")]
    pub for_votes: u128,
    #[abi(uint128, name = "againstVotes_")]
    pub against_votes: u128,
    #[abi(uint128, name = "quorumVotes_")]
    pub quorum_votes: u128,
    #[abi(uint8, name = "state_")]
    pub state: u8,
}

#[derive(
Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[opg("Proposal Type")]
enum ProposalState {
    Pending,
    Active,
    Canceled,
    Failed,
    Succeeded,
    Expired,
    Queued,
    Executed,
}


impl ToString for ProposalState {
    fn to_string(&self) -> String {
        match self {
            ProposalState::Pending => "Pending".into(),
            ProposalState::Active => "Active".into(),
            ProposalState::Canceled => "Canceled".into(),
            ProposalState::Failed => "Failed".into(),
            ProposalState::Succeeded => "Succeeded".into(),
            ProposalState::Expired => "Expired".into(),
            ProposalState::Queued => "Queued".into(),
            ProposalState::Executed => "Executed".into(),
        }
    }
}

impl FromStr for ProposalState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "canceled" => Ok(Self::Canceled),
            "failed" => Ok(Self::Failed),
            "succeeded" => Ok(Self::Succeeded),
            "expired" => Ok(Self::Expired),
            "queued" => Ok(Self::Queued),
            "executed" => Ok(Self::Executed),
            &_ => Err(anyhow::Error::msg(format!("invalid event type {}", s))),
        }
    }
}