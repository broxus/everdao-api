use crate::models::{ProposalFromDb, SearchProposalsRequest, SearchVotesRequest, VoteFromDb};
use crate::services::Services;

impl Services {
    pub async fn search_proposals(
        &self,
        input: SearchProposalsRequest,
    ) -> Result<(Vec<ProposalFromDb>, i64), anyhow::Error> {
        self.sqlx_client.search_proposals(input).await
    }
    pub async fn search_proposals_with_votes(
        &self,
        address: String,
        input: SearchProposalsRequest,
    ) -> Result<(Vec<(ProposalFromDb, VoteFromDb)>, i64), anyhow::Error> {
        self.sqlx_client
            .search_proposals_with_votes(address, input)
            .await
    }

    pub async fn search_votes(
        &self,
        input: SearchVotesRequest,
    ) -> Result<(Vec<VoteFromDb>, i64), anyhow::Error> {
        self.sqlx_client.search_votes(input).await
    }
}
