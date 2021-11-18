#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AbiType {
    Ton,
    Ethereum,
}
