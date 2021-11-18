use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub vaults: Vec<Vault>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    pub id: String,
    pub deposits: Vec<Deposit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    pub account: Account,
    pub amount: String,
    #[serde(rename = "recipientAddr")]
    pub recipient_addr: String,
    pub timestamp: String,
    pub transaction: Transaction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "logIndex")]
    pub log_index: String,
    pub hash: String,
}
