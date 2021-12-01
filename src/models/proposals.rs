use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SearchProposal {
    pub proposal_id: Option<i32>,
    pub proposer: Option<String>,
    pub description: String,
    pub start_time_ge: Option<i32>,
    pub start_time_le: Option<i32>,
    pub end_time_ge: Option<i32>,
    pub end_time_le: Option<i32>,
    pub state: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct CreateProposal {
    pub proposal_id: i32,
    pub proposer: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
    pub execution_time: i32,
    pub for_votes: Decimal,
    pub against_votes: Decimal,
    pub quorum_votes: Decimal,
    pub state: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdateProposal {
    pub for_votes: Option<Decimal>,
    pub against_votes: Option<Decimal>,
    pub quorum_votes: Option<Decimal>,
    pub state: Option<String>,
}

