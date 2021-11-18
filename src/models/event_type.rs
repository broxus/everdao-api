use crate::models::abi::staking::{Deposit, RewardClaimed, Withdraw};
use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Event Type")]
pub enum EventType {
    Deposit,
    Withdraw,
    Claim,
    Freeze,
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::Deposit => "Deposit".into(),
            EventType::Withdraw => "Withdraw".into(),
            EventType::Claim => "Claim".into(),
            EventType::Freeze => "Freeze".into(),
        }
    }
}

impl FromStr for EventType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "deposit" => Ok(Self::Deposit),
            "withdraw" => Ok(Self::Withdraw),
            "claim" => Ok(Self::Claim),
            "freeze" => Ok(Self::Freeze),
            &_ => Err(anyhow::Error::msg(format!("invalid event type {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BalanceEvent {
    Deposit(Deposit),
    Withdraw(Withdraw),
    Claim(RewardClaimed),
}
