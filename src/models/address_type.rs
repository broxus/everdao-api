use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Event Type")]
pub enum UserAddressType {
    TonPubkey,
    EthAddress,
}

impl ToString for UserAddressType {
    fn to_string(&self) -> String {
        match self {
            UserAddressType::TonPubkey => "TonPubkey".into(),
            UserAddressType::EthAddress => "EthAddress".into(),
        }
    }
}

impl FromStr for UserAddressType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tonpubkey" => Ok(Self::TonPubkey),
            "ethaddress" => Ok(Self::EthAddress),
            &_ => Err(anyhow::Error::msg(format!(
                "invalid user address type {}",
                s
            ))),
        }
    }
}
