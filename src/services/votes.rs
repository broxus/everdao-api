use crate::models::*;
use crate::services::*;

impl Services {
    pub async fn search_votes(
        &self,
        input: VotesSearch,
    ) -> Result<(impl Iterator<Item = VoteFromDb>, i64), anyhow::Error> {
        let votes = self.sqlx_client.search_votes(input.clone()).await?;
        let total_count = self.sqlx_client.votes_total_count(input).await?;

        Ok((votes, total_count))
    }
}
