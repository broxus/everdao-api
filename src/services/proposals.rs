use crate::models::*;
use crate::services::*;

impl Services {
    pub async fn get_proposal(&self, id: u32) -> Result<Option<ProposalFromDb>, anyhow::Error> {
        self.sqlx_client.get_proposal(id).await
    }

    pub async fn search_proposals(
        &self,
        input: ProposalsSearch,
    ) -> Result<(impl Iterator<Item = ProposalFromDb>, i64), anyhow::Error> {
        let proposals = self.sqlx_client.search_proposals(input.clone()).await?;
        let total_count = self.sqlx_client.proposals_total_count(input).await?;

        Ok((proposals, total_count))
    }

    pub async fn search_voter_proposals(
        &self,
        address: String,
        mut input: ProposalsSearch,
    ) -> Result<(impl Iterator<Item = ProposalFromDb>, i64), anyhow::Error> {
        input.data.filters.voter = Some(address);
        let proposals = self.sqlx_client.search_proposals(input.clone()).await?;
        let total_count = self.sqlx_client.proposals_total_count(input).await?;

        Ok((proposals, total_count))
    }
}
