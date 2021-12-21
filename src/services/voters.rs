use crate::models::*;
use crate::services::*;
use crate::utils::*;

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
            .proposals_with_votes_total_count(address, input.data.filters)
            .await?;

        Ok((proposals_with_votes, total_count))
    }

    pub async fn proposals_count(
        &self,
        voters: Vec<String>,
    ) -> Result<impl Iterator<Item = (String, i64)>, anyhow::Error> {
        let input: VotersProposalsCountSearch = VotersProposalsCountFilters {
            voters: Some(voters),
        }
        .ordered(None)
        .paginated(100, 0);

        self.sqlx_client.proposals_count_search(input).await
    }

    pub async fn search_proposals_count(
        &self,
        input: VotersProposalsCountSearch,
    ) -> Result<impl Iterator<Item = (String, i64)>, anyhow::Error> {
        self.sqlx_client.proposals_count_search(input).await
    }
}
