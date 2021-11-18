use crate::models::sqlx::RewardRoundInfoFromDb;
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn new_reward_round(
        &self,
        new_reward_round: RewardRoundInfoFromDb,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        //update last round
        if new_reward_round.num_round > 0 {
            sqlx::query!(
                r#"UPDATE reward_rounds SET end_time = $1 WHERE num_round = $2"#,
                new_reward_round.start_time,
                new_reward_round.num_round - 1
            )
            .execute(&mut tx)
            .await?;
        }

        sqlx::query!(r#"INSERT INTO reward_rounds (num_round, start_time, end_time, reward_tokens, total_reward) 
        VALUES ($1, $2, $3, $4, $5)"#,
        new_reward_round.num_round,
        new_reward_round.start_time,
        new_reward_round.end_time,
        new_reward_round.reward_tokens,
        new_reward_round.total_reward).execute(&mut tx).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_all_reward_rounds(&self) -> Result<Vec<RewardRoundInfoFromDb>, anyhow::Error> {
        sqlx::query_as!(
            RewardRoundInfoFromDb,
            r#"SELECT * FROM reward_rounds ORDER BY num_round"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }
}
