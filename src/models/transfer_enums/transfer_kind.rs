use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Transfer status")]
pub enum TransferKind {
    TonToEth,
    EthToTon,
}

impl ToString for TransferKind {
    fn to_string(&self) -> String {
        match self {
            TransferKind::TonToEth => "TonToEth".to_string(),
            TransferKind::EthToTon => "EthToTon".to_string(),
        }
    }
}

impl FromStr for TransferKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tontoeth" => Ok(Self::TonToEth),
            "ethtoton" => Ok(Self::EthToTon),
            &_ => Err(anyhow::Error::msg(format!("invalid transfer kind {}", s))),
        }
    }
}
