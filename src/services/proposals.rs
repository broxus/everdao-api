use crate::models::*;
use crate::services::*;

impl Services {
    pub async fn get_proposals(
        &self,
        ids: Vec<u32>,
    ) -> Result<impl Iterator<Item = ProposalFromDb>, anyhow::Error> {
        self.sqlx_client.get_proposals(ids).await
    }

    pub async fn search_proposals(
        &self,
        input: ProposalsSearch,
    ) -> Result<(impl Iterator<Item = ProposalFromDb>, i64), anyhow::Error> {
        let proposals = self.sqlx_client.search_proposals(input.clone()).await?;
        let total_count = self.sqlx_client.proposals_total_count(input).await?;

        Ok((proposals, total_count))
    }
}
