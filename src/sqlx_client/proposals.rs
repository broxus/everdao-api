use chrono::Utc;
use itertools::Itertools;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::Row;

use crate::models::{
    CreateProposal, ProposalFromDb, ProposalOrdering, ProposalState, SearchProposalsRequest,
    UpdateProposalVotes, VoteFromDb,
};
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn search_proposals(
        &self,
        input: SearchProposalsRequest,
    ) -> Result<(Vec<ProposalFromDb>, i64), anyhow::Error> {
        let (updates, args_len, args, mut args_clone) = filter_proposals_query(&input);

        let mut query = "SELECT proposal_id, contract_address, proposer, description, start_time, end_time, execution_time, for_votes,
                  against_votes, quorum_votes, message_hash, transaction_hash, timestamp_block, actions,
                  executed, canceled, queued, grace_period, updated_at, created_at, canceled_at, executed_at, queued_at FROM proposals"
            .to_string();
        if !updates.is_empty() {
            query = format!("{} WHERE {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = "SELECT COUNT(*) FROM proposals".to_string();
        if !updates.is_empty() {
            query_count = format!("{} WHERE {}", query_count, updates.iter().format(" AND "));
        }

        let total_count: i64 = sqlx::query_with(&query_count, args)
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
                canceled_at: x.get(20),
                executed_at: x.get(21),
                queued_at: x.get(22),
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }

    pub async fn search_proposals_with_votes(
        &self,
        address: String,
        mut input: SearchProposalsRequest,
    ) -> Result<(Vec<(ProposalFromDb, VoteFromDb)>, i64), anyhow::Error> {
        input.proposal_id = None;
        let (updates, args_len, args, mut args_clone) = filter_proposals_query(&input);

        let mut query = format!("SELECT proposals.proposal_id, proposals.contract_address, proposals.proposer, proposals.description, proposals.start_time, proposals.end_time, proposals.execution_time, proposals.for_votes,
                  proposals.against_votes, proposals.quorum_votes, proposals.message_hash, proposals.transaction_hash, proposals.timestamp_block, proposals.actions,
                  proposals.executed, proposals.canceled, proposals.queued, proposals.grace_period, proposals.updated_at, proposals.created_at, proposals.canceled_at,
                  proposals.executed_at, proposals.queued_at,
                  votes.proposal_id, votes.voter, votes.support, votes.reason, votes.votes, votes.message_hash, votes.transaction_hash, votes.timestamp_block, votes.created_at
                  FROM proposals inner join votes on proposals.proposal_id = votes.proposal_id
                  WHERE voter = '{}'", address);
        if !updates.is_empty() {
            query = format!("{} {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = format!("SELECT COUNT(*) FROM proposals inner join votes on proposals.proposal_id = votes.proposal_id  WHERE voter = '{}'", address);
        if !updates.is_empty() {
            query_count = format!("{} {}", query_count, updates.iter().format(" AND "));
        }

        let total_count: i64 = sqlx::query_with(&query_count, args)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.get(0))
            .unwrap_or_default();

        let ordering = if let Some(ordering) = input.ordering {
            match ordering {
                ProposalOrdering::CreatedAtDesc => "ORDER BY votes.timestamp_block DESC",
                ProposalOrdering::CreatedAtAsc => "ORDER BY votes.timestamp_block",
            }
        } else {
            "ORDER BY votes.timestamp_block DESC"
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
            .map(|x| {
                (
                    ProposalFromDb {
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
                        canceled_at: x.get(20),
                        executed_at: x.get(21),
                        queued_at: x.get(22),
                    },
                    VoteFromDb {
                        proposal_id: x.get(23),
                        voter: x.get(24),
                        support: x.get(25),
                        reason: x.get(26),
                        votes: x.get(27),
                        message_hash: x.get(28),
                        transaction_hash: x.get(29),
                        timestamp_block: x.get(30),
                        created_at: x.get(31),
                    },
                )
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }

    pub async fn create_proposal(&self, proposal: CreateProposal) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"INSERT INTO proposals (
            proposal_id, contract_address, proposer, description, start_time, end_time, execution_time, for_votes, against_votes,
            quorum_votes, message_hash, transaction_hash, timestamp_block, actions, grace_period)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#,
            proposal.proposal_id,
            proposal.contract_address,
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
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_proposal_votes(
        &self,
        proposal: UpdateProposalVotes,
        proposal_id: i32,
    ) -> Result<(), anyhow::Error> {
        let updated_at = chrono::Utc::now().timestamp();
        sqlx::query!(
            r#"UPDATE proposals SET for_votes = $1, against_votes = $2, updated_at = $3
            WHERE proposal_id = $4
            "#,
            proposal.for_votes,
            proposal.against_votes,
            updated_at,
            proposal_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_proposal_executed(
        &self,
        proposal_id: i32,
        timestamp_block: i32,
    ) -> Result<(), anyhow::Error> {
        let updated_at = chrono::Utc::now().timestamp();
        sqlx::query!(
            r#"UPDATE proposals SET executed = true, executed_at = $1, updated_at = $2
            WHERE proposal_id = $3"#,
            timestamp_block,
            updated_at,
            proposal_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_proposal_canceled(
        &self,
        proposal_id: i32,
        timestamp_block: i32,
    ) -> Result<(), anyhow::Error> {
        let updated_at = chrono::Utc::now().timestamp();
        sqlx::query!(
            r#"UPDATE proposals SET canceled = true, canceled_at = $1, updated_at = $2
            WHERE proposal_id = $3"#,
            timestamp_block,
            updated_at,
            proposal_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_proposal_queued(
        &self,
        execution_time: i64,
        proposal_id: i32,
        timestamp_block: i32,
    ) -> Result<(), anyhow::Error> {
        let updated_at = chrono::Utc::now().timestamp();
        sqlx::query!(
            r#"UPDATE proposals SET queued = true, execution_time = $1, queued_at = $2, updated_at = $3
            WHERE proposal_id = $4
            "#,
            execution_time,
            timestamp_block,
            updated_at,
            proposal_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
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
        args_clone.add(proposal_id)
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
