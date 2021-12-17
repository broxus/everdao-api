use anyhow::Result;
use chrono::Utc;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

impl SqlxClient {
    pub async fn search_proposals_with_votes(
        &self,
        address: String,
        input: VotersSearch,
    ) -> Result<impl Iterator<Item = (ProposalFromDb, VoteFromDb)> + Send + Sync> {
        let mut query = OwnedPartBuilder::new().starts_with(
            "SELECT \
            proposals.id, proposals.address, proposals.proposer, proposals.description, proposals.start_time, \
            proposals.end_time, proposals.execution_time, proposals.grace_period,  proposals.time_lock, \
            proposals.voting_delay, proposals.for_votes, proposals.against_votes, proposals.quorum_votes, \
            proposals.message_hash, proposals.transaction_hash, proposals.timestamp_block, proposals.actions, \
            proposals.executed, proposals.canceled, proposals.queued, proposals.executed_at, proposals.canceled_at, \
            proposals.queued_at, proposals.updated_at, proposals.created_at,\
            votes.proposal_id, votes.voter, votes.support, votes.reason, votes.votes, votes.locked, votes.message_hash, \
            votes.transaction_hash, votes.timestamp_block, votes.created_at \
            FROM proposals INNER JOIN votes on proposals.id = votes.proposal_id");

        let mut args_len = 0;

        query
            .push_part(voter_filters(address, input.data.filters, &mut args_len))
            .push(voters_ordering(input.data.ordering))
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

        Ok(proposals.into_iter().map(RowReader::from_row).map(|mut x| {
            (
                ProposalFromDb {
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
                },
                VoteFromDb {
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
                },
            )
        }))
    }

    pub async fn proposals_with_votes_total_count(
        &self,
        address: String,
        input: VotersSearch,
    ) -> Result<i64> {
        let mut args_len = 0;

        let mut query = OwnedPartBuilder::new().starts_with(
            "SELECT COUNT(*) FROM proposals INNER JOIN votes on proposals.id = votes.proposal_id",
        );

        query.push_part(voter_filters(address, input.data.filters, &mut args_len));

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

fn voter_filters(address: String, filters: VoterFilters, args_len: &mut u32) -> impl QueryPart {
    WhereAndConditions((
        {
            *args_len += 1;
            Some((format!("voter = ${}", *args_len), address))
        },
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
        filters.support.map(|support| {
            *args_len += 1;
            (format!("support = ${}", *args_len), support)
        }),
        filters.locked.map(|locked| {
            *args_len += 1;
            (format!("locked = ${}", *args_len), locked)
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

fn voters_ordering(ordering: Option<VotersOrdering>) -> &'static str {
    let VotersOrdering { column, direction } = ordering.unwrap_or_default();

    match (column, direction) {
        (VoterColumn::CreatedAt, Direction::Ascending) => "ORDER BY votes.timestamp_block",
        (VoterColumn::CreatedAt, Direction::Descending) => "ORDER BY votes.timestamp_block DESC",
    }
}

fn max_limit(limit: u32) -> u32 {
    std::cmp::min(limit, 100)
}
