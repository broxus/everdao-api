use crate::models::*;
use crate::services::*;

impl Services {
    pub async fn search_proposals_with_votes(
        &self,
        address: String,
        input: VotersSearch,
    ) -> Result<(impl Iterator<Item = (ProposalFromDb, VoteFromDb)>, i64), anyhow::Error> {
        let proposals_with_votes = self
            .sqlx_client
            .search_proposals_with_votes(address.clone(), input.clone())
            .await?;
        let total_count = self
            .sqlx_client
            .proposals_with_votes_total_count(address, input)
            .await?;

        Ok((proposals_with_votes, total_count))
    }
}
