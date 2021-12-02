use chrono::Utc;
use itertools::Itertools;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::Row;

use crate::models::{ProposalFromDb, ProposalOrdering, ProposalState, SearchProposalsRequest};
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn search_proposals(
        &self,
        input: SearchProposalsRequest,
    ) -> Result<(Vec<ProposalFromDb>, i32), anyhow::Error> {
        let (updates, args_len, args, mut args_clone) = filter_proposals_query(&input);

        let mut query = "SELECT proposal_id, proposer, description, start_time, end_time, execution_time, for_votes,
                  against_votes, quorum_votes, message_hash, transaction_hash, timestamp_block, actions,
                  executed, canceled, queued, grace_period, updated_at, created_at FROM proposals"
            .to_string();
        if !updates.is_empty() {
            query = format!("{} WHERE {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = "SELECT COUNT(*) FROM proposals".to_string();
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
                ProposalOrdering::CreatedAtDesc => "ORDER BY timestamp_block DESC",
                ProposalOrdering::CreatedAtAsc => "ORDER BY timestamp_block",
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
            .map(|x| ProposalFromDb {
                proposal_id: x.get(0),
                contract_address: x.get(1),
                proposer: x.get(2),
                description: x.get(3),
                start_time: x.get(4),
                end_time: x.get(5),
                execution_time: x.get(6),
                for_votes: x.get(7),
                against_votes: x.get(8),
                quorum_votes: x.get(9),
                message_hash: x.get(10),
                transaction_hash: x.get(11),
                timestamp_block: x.get(12),
                actions: x.get(13),
                executed: x.get(14),
                canceled: x.get(15),
                queued: x.get(16),
                grace_period: x.get(17),
                updated_at: x.get(18),
                created_at: x.get(19),
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }

    pub async fn create_proposal(
        &self,
        proposal: CreatePropposal,
    ) -> Result<ProposalFromDb, anyhow::Error> {
        sqlx::query!(
            r#"INSERT INTO proposal (
            proposal_id, proposer, description, start_time, end_time, execution_time, for_votes, against_votes,
            quorum_votes, message_hash, transaction_hash, timestamp_block, actions, grace_period)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING
                  proposal_id, proposer, description, start_time, end_time, execution_time, for_votes,
                  against_votes, quorum_votes, message_hash, transaction_hash, timestamp_block, actions,
                  executed, canceled, queued, grace_period, updated_at, created_at"#,
            proposal.proposal_id,
            proposal.proposer,
            proposal.description,
            proposal.start_time,
            proposal.end_time,
            proposal.execution_time,
            proposal.for_votes,
            proposal.against_votes,
            proposal.quorum_votes,
            proposal.message_hash,
            proposal.transaction_hash,
            proposal.timestamp_block,
            serde_json::to_value(proposal.actions).unwrap(),
            proposal.grace_period,
        )
        .fetch_one(&self.pool)
        .await
    }
}

pub fn filter_proposals_query(
    input: &SearchProposalsRequest,
) -> (Vec<String>, i32, PgArguments, PgArguments) {
    let SearchProposalsRequest {
        proposal_id,
        proposer,
        start_time_ge,
        start_time_le,
        end_time_ge,
        end_time_le,
        state,
        ..
    } = input.clone();

    let mut args = PgArguments::default();
    let mut args_clone = PgArguments::default();
    let mut updates = Vec::new();
    let mut args_len = 0;

    if let Some(proposal_id) = proposal_id {
        updates.push(format!("proposal_id = ${}", args_len + 1,));
        args_len += 1;
        args.add(proposal_id);
        args_clone.add(proposal_kind)
    }

    if let Some(proposer) = proposer {
        updates.push(format!("proposer = ${}", args_len + 1,));
        args_len += 1;
        args.add(proposer.clone());
        args_clone.add(proposer);
    }

    if let Some(start_time_ge) = start_time_ge {
        updates.push(format!("start_time >= ${}", args_len + 1,));
        args_len += 1;
        args.add(start_time_ge);
        args_clone.add(start_time_ge)
    }

    if let Some(start_time_le) = start_time_le {
        updates.push(format!("start_time <= ${}", args_len + 1,));
        args_len += 1;
        args.add(start_time_le);
        args_clone.add(start_time_le)
    }

    if let Some(end_time_ge) = end_time_ge {
        updates.push(format!("end_time >= ${}", args_len + 1,));
        args_len += 1;
        args.add(end_time_ge);
        args_clone.add(end_time_ge)
    }

    if let Some(end_time_le) = end_time_le {
        updates.push(format!("end_time <= ${}", args_len + 1,));
        args_len += 1;
        args.add(end_time_le);
        args_clone.add(end_time_le)
    }

    if let Some(state) = state {
        let now = Utc::now().timestamp();
        match state {
            ProposalState::Pending => {
                updates.push(format!("start_time >= ${}", args_len + 1,));
                args_len += 1;
                args.add(now);
                args_clone.add(now)
            }
            ProposalState::Active => {
                updates.push(format!("start_time <= ${}", args_len + 1,));
                args_len += 1;
                args.add(now);
                args_clone.add(now);

                updates.push(format!("end_time > ${}", args_len + 1,));
                args_len += 1;
                args.add(now);
                args_clone.add(now);
            }
            ProposalState::Canceled => {
                updates.push(format!("canceled = true"));
            }
            ProposalState::Executed => {
                updates.push(format!("executed = true"));
            }
            ProposalState::Failed => {
                updates.push(format!("end_time < ${}", args_len + 1,));
                args_len += 1;
                args.add(now);
                args_clone.add(now);

                updates.push(format!(
                    "(for_votes <= against_votes OR for_votes < quorum_votes)"
                ));
            }
            ProposalState::Succeeded => {
                updates.push(format!("end_time < ${}", args_len + 1,));
                args_len += 1;
                args.add(now);
                args_clone.add(now);

                updates.push(format!(
                    "(for_votes > against_votes AND for_votes >= quorum_votes AND queued = false)"
                ));
            }
            ProposalState::Expired => {
                updates.push(format!(
                    "(execution_time + grace_period) < ${}",
                    args_len + 1,
                ));
                args_len += 1;
                args.add(now);
                args_clone.add(now);

                updates.push(format!(
                    "(for_votes > against_votes AND for_votes >= quorum_votes AND queued = true AND executed = false)"
                ));
            }
            ProposalState::Queued => {
                updates.push(format!(
                    "(execution_time + grace_period) > ${}",
                    args_len + 1,
                ));
                args_len += 1;
                args.add(now);
                args_clone.add(now);

                updates.push(format!(
                    "(for_votes > against_votes AND for_votes >= quorum_votes AND queued = true AND executed = false)"
                ));
            }
        }
    }

    (updates, args_len, args, args_clone)
}
