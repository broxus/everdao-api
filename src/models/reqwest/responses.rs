use rust_decimal::Decimal;
use serde::de::{self, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct CurrencyInfoResponse {
    pub currency: String,
    pub currency_scale: i32,
    pub currency_price: Decimal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct TokensInfoResponse {
    pub name: String,
    pub token: HashMap<String, TokenInfo>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct TokenInfo {
    pub proxy: String,
    pub vaults: Vec<VaultInfo>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct VaultInfo {
    #[serde(rename = "chainId", deserialize_with = "from_str")]
    pub chain_id: i32,
    pub vault: String,
    #[serde(rename = "ethereumConfiguration")]
    pub ethereum_configuration: String,
    #[serde(rename = "depositType")]
    pub deposit_type: String,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
