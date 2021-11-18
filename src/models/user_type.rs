use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Event Type")]
pub enum UserType {
    Ordinary,
    Relay,
}

impl ToString for UserType {
    fn to_string(&self) -> String {
        match self {
            UserType::Ordinary => "Ordinary".into(),
            UserType::Relay => "Relay".into(),
        }
    }
}

impl FromStr for UserType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ordinary" => Ok(Self::Ordinary),
            "relay" => Ok(Self::Relay),
            &_ => Err(anyhow::Error::msg(format!(
                "invalid user address type {}",
                s
            ))),
        }
    }
}
