use crate::models::*;
use crate::services::*;

impl Services {
    pub async fn search_proposals(
        &self,
        input: ProposalsSearch,
    ) -> Result<(impl Iterator<Item = ProposalFromDb>, i64), anyhow::Error> {
        let proposals = self.sqlx_client.search_proposals(input.clone()).await?;
        let total_count = self
            .sqlx_client
            .proposals_total_count(input.data.filters)
            .await?;

        Ok((proposals, total_count))
    }
    pub async fn overview(&self) -> Result<ProposalsOverview, anyhow::Error> {
        let proposals_total_count = self
            .sqlx_client
            .proposals_total_count(ProposalFilters::default())
            .await?;

        Ok(ProposalsOverview {
            proposals_total_count,
        })
    }
}
