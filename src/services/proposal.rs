use chrono::Utc;
use nekoton_abi::num_traits::{One, Zero};
use sqlx::types::Decimal;

use crate::api::requests::{
    GraphRequest, SearchProposalsRequest, SearchTransactionsRequest, SearchVotesRequest,
    UserPageStakingRequest,
};
use crate::api::responses::{MainPageStakingResponse, UserPageStakingResponse};
use crate::models::sqlx::{GraphDataFromDb, ProposalFromDb, VoteFromDb};
use crate::services::Services;
use crate::sqlx_client::graph_data::fill_graph;

impl Services {
    pub async fn search_proposals(
        &self,
        input: SearchProposalsRequest,
    ) -> Result<(Vec<ProposalFromDb>, i32), anyhow::Error> {
        self.sqlx_client.search_proposals(input).await
    }

    pub async fn search_votes(
        &self,
        input: SearchVotesRequest,
    ) -> Result<(Vec<VoteFromDb>, i32), anyhow::Error> {
        self.sqlx_client.search_votes(input).await
    }
}
