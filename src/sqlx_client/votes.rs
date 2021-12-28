use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

impl SqlxClient {
    pub async fn create_vote(&self, vote: CreateVote) -> Result<()> {
        let locked = true;

        sqlx::query!(
            r#"INSERT INTO votes (proposal_id, voter, support, reason, votes, message_hash, transaction_hash, timestamp_block, locked)
                          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            vote.proposal_id,
            vote.voter,
            vote.support,
            vote.reason,
            vote.votes,
            vote.message_hash,
            vote.transaction_hash,
            vote.timestamp_block,
            locked
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn unlock_vote(&self, vote: UnlockVote) -> Result<VoteFromDb> {
        let locked = false;

        sqlx::query_as!(
            VoteFromDb,
            r#"
            UPDATE votes SET locked = $1
            WHERE proposal_id = $2 AND voter = $3
            RETURNING proposal_id,
                voter,
                support,
                reason,
                votes,
                locked,
                message_hash,
                transaction_hash,
                timestamp_block,
                created_at"#,
            locked,
            vote.proposal_id,
            vote.voter,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
    }

    pub async fn search_votes(
        &self,
        input: VotesSearch,
    ) -> Result<impl Iterator<Item = VoteFromDb> + Send + Sync> {
        let mut args_len = 0;

        let mut query = OwnedPartBuilder::new().starts_with(
            "SELECT \
                proposal_id, voter, support, reason, votes, locked, message_hash, transaction_hash, \
                timestamp_block, created_at \
            FROM votes",
        );

        query
            .push_part(vote_filters(input.data.filters, &mut args_len))
            .push(votes_ordering(input.data.ordering))
            .push_with_arg(
                {
                    format!("LIMIT ${}", {
                        args_len += 1;
                        args_len
                    })
                },
                max_limit(input.limit),
            )
            .push_with_arg(
                {
                    format!("OFFSET ${}", {
                        args_len += 1;
                        args_len
                    })
                },
                input.offset,
            );

        let (query, args) = query.split();

        let votes = sqlx::query_with(&query, args).fetch_all(&self.pool).await?;

        Ok(votes
            .into_iter()
            .map(RowReader::from_row)
            .map(|mut x| VoteFromDb {
                proposal_id: x.read_next(),
                voter: x.read_next(),
                support: x.read_next(),
                reason: x.read_next(),
                votes: x.read_next(),
                locked: x.read_next(),
                message_hash: x.read_next(),
                transaction_hash: x.read_next(),
                timestamp_block: x.read_next(),
                created_at: x.read_next(),
            }))
    }

    pub async fn votes_total_count(&self, input: VoteFilters) -> Result<i64> {
        let mut args_len = 0;

        let mut query = OwnedPartBuilder::new().starts_with("SELECT COUNT(*) FROM votes");

        query.push_part(vote_filters(input, &mut args_len));

        let (query, args) = query.split();

        let total_count: i64 = sqlx::query_with(&query, args)
            .fetch_one(&self.pool)
            .await
            .map(RowReader::from_row)
            .map(|mut x| x.read_next())
            .unwrap_or_default();

        Ok(total_count)
    }
}

fn vote_filters(filters: VoteFilters, args_len: &mut u32) -> impl QueryPart {
    WhereAndConditions((
        filters.voter.map(|address| {
            *args_len += 1;
            (format!("voter = ${}", *args_len), address)
        }),
        filters.proposal_id.map(|proposal_id| {
            *args_len += 1;
            (format!("proposal_id = ${}", *args_len), proposal_id)
        }),
        filters.support.map(|support| {
            *args_len += 1;
            (format!("support = ${}", *args_len), support)
        }),
        filters.locked.map(|locked| {
            *args_len += 1;
            (format!("locked = ${}", *args_len), locked)
        }),
    ))
}

fn votes_ordering(ordering: Option<VotesOrdering>) -> &'static str {
    let VotesOrdering { column, direction } = ordering.unwrap_or_default();

    match (column, direction) {
        (VoteColumn::CreatedAt, Direction::Ascending) => "ORDER BY timestamp_block",
        (VoteColumn::CreatedAt, Direction::Descending) => "ORDER BY timestamp_block DESC",
    }
}

fn max_limit(limit: u32) -> u32 {
    std::cmp::min(limit, 100)
}
