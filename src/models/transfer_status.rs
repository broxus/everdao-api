use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Transfer status")]
pub enum TransferStatus {
    Pending,
    Rejected,
    Confirmed,
}

impl ToString for TransferStatus {
    fn to_string(&self) -> String {
        match self {
            TransferStatus::Pending => "Pending".to_string(),
            TransferStatus::Rejected => "Rejected".to_string(),
            TransferStatus::Confirmed => "Confirmed".to_string(),
        }
    }
}

impl FromStr for TransferStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "rejected" => Ok(Self::Rejected),
            "confirmed" => Ok(Self::Confirmed),
            &_ => Err(anyhow::Error::msg(format!("invalid transfer status {}", s))),
        }
    }
}
