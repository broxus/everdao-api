use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[opg("Proposal state")]
pub enum ProposalState {
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
