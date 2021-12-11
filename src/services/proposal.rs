use crate::models::*;
use crate::services::*;

impl Services {
    pub async fn get_proposal(&self, id: u32) -> Result<Option<ProposalFromDb>, anyhow::Error> {
        self.sqlx_client.get_proposal(id).await
    }

    pub async fn search_proposals(
        &self,
        input: ProposalsSearch,
    ) -> Result<impl Iterator<Item = ProposalFromDb>, anyhow::Error> {
        self.sqlx_client.search_proposals(input).await
    }

    pub async fn search_votes(
        &self,
        input: VotesSearch,
    ) -> Result<impl Iterator<Item = VoteFromDb>, anyhow::Error> {
        self.sqlx_client.search_votes(input).await
    }

    pub async fn search_voter_proposals(
        &self,
        address: String,
        mut input: ProposalsSearch,
    ) -> Result<impl Iterator<Item = ProposalFromDb>, anyhow::Error> {
        input.data.filters.voter = Some(address);
        self.sqlx_client.search_proposals(input).await
    }
}
