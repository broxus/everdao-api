use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

impl SqlxClient {
    pub async fn create_vote(&self, vote: CreateVote) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO votes (proposal_id, voter, support, reason, votes, message_hash, transaction_hash, timestamp_block)
                          VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            vote.proposal_id,
            vote.voter,
            vote.support,
            vote.reason,
            vote.votes,
            vote.message_hash,
            vote.transaction_hash,
            vote.timestamp_block
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn search_votes(
        &self,
        input: VotesSearch,
    ) -> Result<impl Iterator<Item = VoteFromDb> + Send + Sync> {
        let mut query = OwnedPartBuilder::new().starts_with(
            "SELECT \
                proposal_id, voter, support, reason, votes, message_hash, transaction_hash, \
                timestamp_block, created_at \
            FROM votes",
        );

        let mut args_len = 0;

        query
            .push_part(proposal_filters(input.data.filters, &mut args_len))
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
                message_hash: x.read_next(),
                transaction_hash: x.read_next(),
                timestamp_block: x.read_next(),
                created_at: x.read_next(),
            }))
    }
}

fn proposal_filters(filters: VoteFilters, args_len: &mut u32) -> impl QueryPart {
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
    ))
}

fn votes_ordering(ordering: Option<VotesOrdering>) -> &'static str {
    let VotesOrdering { column, direction } = ordering.unwrap_or_default();

    match (column, direction) {
        (VoteColumn::CreatedAt, Direction::Ascending) => "ORDER BY created_at",
        (VoteColumn::CreatedAt, Direction::Descending) => "ORDER BY created_at DESC",
        (VoteColumn::UpdatedAt, Direction::Ascending) => "ORDER BY updated_at",
        (VoteColumn::UpdatedAt, Direction::Descending) => "ORDER BY updated_at DESC",
    }
}

fn max_limit(limit: u32) -> u32 {
    std::cmp::min(limit, 100)
}
