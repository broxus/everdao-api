use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[opg("Ordering")]
pub enum Direction {
    #[serde(rename = "ASC")]
    Ascending,
    #[serde(rename = "DESC")]
    Descending,
}
