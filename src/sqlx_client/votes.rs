use itertools::Itertools;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::Row;

use crate::models::{CreateVote, SearchVotesRequest, VoteFromDb, VoteOrdering};
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn search_votes(
        &self,
        input: SearchVotesRequest,
    ) -> Result<(Vec<VoteFromDb>, i32), anyhow::Error> {
        let (updates, args_len, args, mut args_clone) = filter_transactions_query(&input);

        let mut query = "SELECT
         proposal_id, voter, support, reason, votes, message_hash, transaction_hash, timestamp_block, created_at
         FROM votes"
            .to_string();
        if !updates.is_empty() {
            query = format!("{} WHERE {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = "SELECT COUNT(*) FROM votes".to_string();
        if !updates.is_empty() {
            query_count = format!("{} WHERE {}", query_count, updates.iter().format(" AND "));
        }

        let total_count: i32 = sqlx::query_with(&query_count, args)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.get(0))
            .unwrap_or_default();

        let ordering = if let Some(ordering) = input.ordering {
            match ordering {
                VoteOrdering::CreatedAtAsc => "ORDER BY timestamp_block",
                VoteOrdering::CreatedAtDesc => "ORDER BY timestamp_block DESC",
            }
        } else {
            "ORDER BY timestamp_block DESC"
        };

        query = format!(
            "{} {} OFFSET ${} LIMIT ${}",
            query,
            ordering,
            args_len + 1,
            args_len + 2
        );

        args_clone.add(input.offset);
        args_clone.add(input.limit);

        let transactions = sqlx::query_with(&query, args_clone)
            .fetch_all(&self.pool)
            .await?;

        let res = transactions
            .into_iter()
            .map(|x| VoteFromDb {
                proposal_id: x.get(0),
                voter: x.get(1),
                support: x.get(2),
                reason: x.get(3),
                votes: x.get(4),
                message_hash: x.get(5),
                transaction_hash: x.get(6),
                timestamp_block: x.get(7),
                created_at: x.get(8),
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }

    pub async fn create_vote(&self, vote: CreateVote) -> Result<VoteFromDb, anyhow::Error> {
        sqlx::query!(
            r#"INSERT INTO votes (proposal_id, voter, support, reason, votes, message_hash, transaction_hash, timestamp_block)
                          VALUES ($1, $2, $3, $4, $5, $6, $7, &8)
                          RETURNING proposal_id, voter, support, reason, votes, message_hash, transaction_hash, timestamp_block, created_at"#,
            vote.proposal_id,
            vote.voter,
            vote.support,
            vote.reason,
            vote.votes,
            vote.message_hash,
            vote.transaction_hash,
            vote.timestamp_block
        )
        .fetch_one(&self.pool)
        .await
    }
}

pub fn filter_transactions_query(
    input: &SearchVotesRequest,
) -> (Vec<String>, i32, PgArguments, PgArguments) {
    let SearchVotesRequest {
        proposal_id, voter, ..
    } = input.clone();

    let mut args = PgArguments::default();
    let mut args_clone = PgArguments::default();
    let mut updates = Vec::new();
    let mut args_len = 0;

    if let Some(proposal_id) = proposal_id {
        updates.push(format!("proposal_id = ${}", args_len + 1,));
        args_len += 1;
        args.add(proposal_id);
        args_clone.add(proposal_id);
    }

    if let Some(voter) = voter {
        updates.push(format!("voter = ${}", args_len + 1,));
        args_len += 1;
        args.add(voter.clone());
        args_clone.add(voter);
    }

    (updates, args_len, args, args_clone)
}
