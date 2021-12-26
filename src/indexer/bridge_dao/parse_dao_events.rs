use anyhow::Context;
use indexer_lib::TransactionExt;
use itertools::Itertools;
use nekoton_abi::*;
use nekoton_utils::TrustMe;
use sqlx::types::Decimal;
use ton_block::{MsgAddressInt, Transaction};
use ton_consumer::TransactionProducer;

use crate::global_cache::*;
use crate::models::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;
use crate::utils::*;

pub async fn parse_proposal_created_event(
    data: ProposalCreated,
    message_hash: Vec<u8>,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found new proposal : {:?}", data);
    log::debug!(
        "Transaction DAO ROOT address: {}",
        transaction.contract_address()?.to_string()
    );

    if transaction.contract_address()? != *super::DAO_ROOT_ADDRESS {
        // skip event
        return Ok(());
    }

    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    // get expected proposal address
    let dao_root_address = transaction.contract_address()?;
    let function_output = transaction_producer
        .run_local(
            &dao_root_address,
            expected_proposal_address(),
            &[
                answer_id(),
                data.proposal_id.token_value().named("proposalId"),
            ],
        )
        .await?
        .context("none function output")?;
    let proposal_address: MsgAddressInt =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    // get proposal overview
    let function_output = poll_run_local(
        transaction_producer,
        &proposal_address,
        get_overview(),
        &[answer_id()],
        60,
    )
    .await?;
    let proposal_overview: ProposalOverview =
        function_output.tokens.unwrap_or_default().unpack()?;

    // get proposal config
    let function_output = poll_run_local(
        transaction_producer,
        &proposal_address,
        get_config(),
        &[answer_id()],
        60,
    )
    .await?;
    let proposal_config: ProposalConfig =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    let for_votes = sqlx_client
        .votes_sum((data.proposal_id, true).into())
        .await?;

    let against_votes = sqlx_client
        .votes_sum((data.proposal_id, false).into())
        .await?;

    let proposal = CreateProposal {
        id: data.proposal_id as i32,
        address: proposal_address.to_string(),
        proposer: proposal_overview.proposer.to_string(),
        description: proposal_overview.description,
        start_time: proposal_overview.start_time as i64,
        end_time: proposal_overview.end_time as i64,
        execution_time: proposal_overview.execution_time as i64,
        grace_period: proposal_config.grace_period as i64,
        time_lock: proposal_config.time_lock as i64,
        voting_delay: proposal_config.voting_delay as i64,
        for_votes,
        against_votes,
        quorum_votes: Decimal::from(proposal_overview.quorum_votes),
        message_hash,
        transaction_hash,
        timestamp_block,
        actions: ProposalActions {
            ton_actions: data
                .ton_actions
                .into_iter()
                .map(TryFrom::try_from)
                .try_collect()?,
            eth_actions: data.eth_actions.into_iter().map(From::from).collect(),
        },
    };

    sqlx_client.create_proposal(proposal).await?;

    let proposal_actions = remove_proposal_actions_from_cache(&proposal_address)?;
    for proposal_action in proposal_actions {
        match proposal_action {
            ProposalActionType::Executed(timestamp_block) => {
                sqlx_client
                    .update_proposal_executed(proposal_address.to_string(), timestamp_block)
                    .await?;
            }
            ProposalActionType::Canceled(timestamp_block) => {
                sqlx_client
                    .update_proposal_canceled(proposal_address.to_string(), timestamp_block)
                    .await?;
            }
            ProposalActionType::Queued(timestamp_block, execution_time) => {
                sqlx_client
                    .update_proposal_queued(
                        proposal_address.to_string(),
                        timestamp_block,
                        execution_time,
                    )
                    .await?;
            }
        }
    }

    Ok(())
}
