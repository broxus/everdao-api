#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Transactions ordering")]
pub enum TransactionOrdering {
    AmountAscending,
    AmountDescending,
    TimestampBlockAscending,
    TimestampBlockAtDescending,
}
