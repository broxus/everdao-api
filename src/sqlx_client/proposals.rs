use anyhow::Result;
use chrono::Utc;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

impl SqlxClient {
    pub async fn create_proposal(&self, proposal: CreateProposal) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO proposals (
            id, address, proposer, description, start_time, end_time, execution_time, grace_period, time_lock, voting_delay, for_votes,
            against_votes, quorum_votes, message_hash, transaction_hash, timestamp_block, actions)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
            proposal.id,
            proposal.address,
            proposal.proposer,
            proposal.description,
            proposal.start_time,
            proposal.end_time,
            proposal.execution_time,
            proposal.grace_period,
            proposal.time_lock,
            proposal.voting_delay,
            proposal.for_votes,
            proposal.against_votes,
            proposal.quorum_votes,
            proposal.message_hash,
            proposal.transaction_hash,
            proposal.timestamp_block,
            serde_json::to_value(proposal.actions).unwrap(),
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_proposal_executed(
        &self,
        proposal_id: i32,
        timestamp_block: i32,
    ) -> Result<i64> {
        let updated_at = chrono::Utc::now().timestamp();
        let row: (i64,) = sqlx::query_as(
            "WITH pr AS (UPDATE proposals SET executed = true, executed_at = $1, updated_at = $2 \
                WHERE id = $3 RETURNING 1) \
            SELECT count(*) FROM pr",
        )
        .bind(timestamp_block)
        .bind(updated_at)
        .bind(proposal_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    pub async fn update_proposal_canceled(
        &self,
        proposal_id: i32,
        timestamp_block: i32,
    ) -> Result<i64> {
        let updated_at = chrono::Utc::now().timestamp();
        let row: (i64,) = sqlx::query_as(
            "WITH pr AS (UPDATE proposals SET canceled = true, canceled_at = $1, updated_at = $2 \
                WHERE id = $3 RETURNING 1) \
            SELECT count(*) FROM pr",
        )
        .bind(timestamp_block)
        .bind(updated_at)
        .bind(proposal_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    pub async fn update_proposal_queued(
        &self,
        execution_time: i64,
        proposal_id: i32,
        timestamp_block: i32,
    ) -> Result<i64> {
        let updated_at = chrono::Utc::now().timestamp();
        let row: (i64,) = sqlx::query_as(
            "WITH pr AS (UPDATE proposals SET queued = true, execution_time = $1, queued_at = $2, \
                updated_at = $3 WHERE id = $4 RETURNING 1) \
            SELECT count(*) FROM pr",
        )
        .bind(execution_time)
        .bind(timestamp_block)
        .bind(updated_at)
        .bind(proposal_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    pub async fn update_proposal_votes(
        &self,
        proposal_id: i32,
        proposal_votes: UpdateProposalVotes,
    ) -> Result<i64> {
        let updated_at = chrono::Utc::now().timestamp();
        let row: (i64,) = sqlx::query_as(
            "WITH pr AS (UPDATE proposals SET for_votes = for_votes + $2, against_votes = against_votes + $3, \
            updated_at = $4 WHERE id = $1 RETURNING 1) \
            SELECT count(*) FROM pr",
        )
            .bind(proposal_id)
            .bind(proposal_votes.for_votes,)
            .bind(proposal_votes.against_votes,)
            .bind(updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    pub async fn search_proposals(
        &self,
        input: ProposalsSearch,
    ) -> Result<impl Iterator<Item = ProposalFromDb> + Send + Sync> {
        let mut query = OwnedPartBuilder::new().starts_with(
                "SELECT \
                id, address, proposer, description, start_time, end_time, execution_time, \
                grace_period, time_lock, voting_delay, for_votes, against_votes, quorum_votes, message_hash, transaction_hash, \
                timestamp_block, actions, executed, canceled, queued, executed_at, canceled_at, queued_at, \
                updated_at, created_at \
            FROM proposals");

        let mut args_len = 0;

        query
            .push_part(proposal_filters(input.data.filters, &mut args_len))
            .push(proposals_ordering(input.data.ordering))
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

        let proposals = sqlx::query_with(&query, args).fetch_all(&self.pool).await?;

        Ok(proposals
            .into_iter()
            .map(RowReader::from_row)
            .map(|mut x| ProposalFromDb {
                id: x.read_next(),
                address: x.read_next(),
                proposer: x.read_next(),
                description: x.read_next(),
                start_time: x.read_next(),
                end_time: x.read_next(),
                execution_time: x.read_next(),
                grace_period: x.read_next(),
                time_lock: x.read_next(),
                voting_delay: x.read_next(),
                for_votes: x.read_next(),
                against_votes: x.read_next(),
                quorum_votes: x.read_next(),
                message_hash: x.read_next(),
                transaction_hash: x.read_next(),
                timestamp_block: x.read_next(),
                actions: x.read_next(),
                executed: x.read_next(),
                canceled: x.read_next(),
                queued: x.read_next(),
                executed_at: x.read_next(),
                canceled_at: x.read_next(),
                queued_at: x.read_next(),
                updated_at: x.read_next(),
                created_at: x.read_next(),
            }))
    }

    pub async fn proposals_total_count(&self, input: ProposalFilters) -> Result<i64> {
        let mut args_len = 0;

        let mut query = OwnedPartBuilder::new().starts_with("SELECT COUNT(*) FROM proposals");

        query.push_part(proposal_filters(input, &mut args_len));

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

fn proposal_filters(filters: ProposalFilters, args_len: &mut u32) -> impl QueryPart {
    WhereAndConditions((
        filters.start_time_ge.map(|time| {
            *args_len += 1;
            (format!("start_time >= ${}", *args_len), time)
        }),
        filters.start_time_le.map(|time| {
            *args_len += 1;
            (format!("start_time <= ${}", *args_len), time)
        }),
        filters.end_time_ge.map(|time| {
            *args_len += 1;
            (format!("end_time >= ${}", *args_len), time)
        }),
        filters.end_time_le.map(|time| {
            *args_len += 1;
            (format!("end_time <= ${}", *args_len), time)
        }),
        filters.proposal_id.map(|id| {
            *args_len += 1;
            (format!("id = ${}", *args_len), id)
        }),
        filters.proposer.map(|proposer| {
            *args_len += 1;
            (format!("proposer = ${}", *args_len), proposer)
        }),
        filters.proposal_address.map(|proposal_address| {
            *args_len += 1;
            (format!("address = ${}", *args_len), proposal_address)
        }),
        filters.state.map(|state| {
            let now = Utc::now().timestamp();
            match state {
                ProposalState::Pending => {
                    let parts = format!("start_time >= ${}", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Int(now)])
                }
                ProposalState::Active => {
                    let parts = format!(
                        "start_time <= ${} AND end_time > ${}",
                        {
                            *args_len += 1;
                            *args_len
                        },
                        {
                            *args_len += 1;
                            *args_len
                        }
                    );
                    CustomBuild(parts, vec![CustomBuildType::Int(now), CustomBuildType::Int(now)])
                }
                ProposalState::Failed => {
                    let parts = format!("end_time < ${} AND \
                    (for_votes <= against_votes OR for_votes < quorum_votes)", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Int(now)])
                }
                ProposalState::Succeeded => {
                    let parts = format!("end_time < ${} AND \
                    (for_votes > against_votes AND for_votes >= quorum_votes AND queued = false)", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Int(now)])
                }
                ProposalState::Expired => {
                    let parts = format!("(execution_time + grace_period) < ${} AND \
                    (for_votes > against_votes AND for_votes >= quorum_votes AND queued = true AND executed = false)", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Int(now)])
                }
                ProposalState::Queued => {
                    let parts = format!("(execution_time + grace_period) > ${} AND \
                    (for_votes > against_votes AND for_votes >= quorum_votes AND queued = true AND executed = false)", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Int(now)])
                }
                ProposalState::Canceled => {
                    let parts = format!("canceled = ${}", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Bool(true)])
                }
                ProposalState::Executed => {
                    let parts = format!("executed = ${}", {
                        *args_len += 1;
                        *args_len
                    });
                    CustomBuild(parts, vec![CustomBuildType::Bool(true)])
                }
            }
        }),
    ))
}

fn proposals_ordering(ordering: Option<ProposalsOrdering>) -> &'static str {
    let ProposalsOrdering { column, direction } = ordering.unwrap_or_default();

    match (column, direction) {
        (ProposalColumn::CreatedAt, Direction::Ascending) => "ORDER BY timestamp_block",
        (ProposalColumn::CreatedAt, Direction::Descending) => "ORDER BY timestamp_block DESC",
    }
}

fn max_limit(limit: u32) -> u32 {
    std::cmp::min(limit, 100)
}
