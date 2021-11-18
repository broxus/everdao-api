#[derive(Debug, serde::Serialize, Clone)]
pub struct CurrencyInfoRequest {
    pub timestamp_block: i32,
    pub currency_address: String,
}
