use anyhow::Context;
use indexer_lib::TransactionExt;
use nekoton_abi::*;
use ton_block::Transaction;
use ton_consumer::TransactionProducer;

use crate::global_cache::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_proposal_executed_event(
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found proposal executed event");
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    if sqlx_client
        .update_proposal_executed(proposal_address.to_string(), timestamp_block)
        .await?
        == 0
    {
        // get proposal id
        let function_output = node
            .run_local(&proposal_address, get_id(), &[])
            .await?
            .context("none function output")?;
        let proposal_id: u32 = function_output.tokens.unwrap_or_default().unpack_first()?;

        save_proposal_action_in_cache(
            proposal_id as i32,
            ProposalActionType::Executed(timestamp_block),
        )
    }

    Ok(())
}

pub async fn parse_proposal_canceled_event(
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found proposal canceled event");
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    if sqlx_client
        .update_proposal_canceled(proposal_address.to_string(), timestamp_block)
        .await?
        == 0
    {
        // get proposal id
        let function_output = node
            .run_local(&proposal_address, get_id(), &[])
            .await?
            .context("none function output")?;
        let proposal_id: u32 = function_output.tokens.unwrap_or_default().unpack_first()?;

        save_proposal_action_in_cache(
            proposal_id as i32,
            ProposalActionType::Canceled(timestamp_block),
        )
    }

    Ok(())
}

pub async fn parse_proposal_queued_event(
    execution_time: u32,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found proposal queued event");
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    if sqlx_client
        .update_proposal_queued(
            proposal_address.to_string(),
            timestamp_block,
            execution_time as i64,
        )
        .await?
        == 0
    {
        // get proposal id
        let function_output = node
            .run_local(&proposal_address, get_id(), &[])
            .await?
            .context("none function output")?;
        let proposal_id: u32 = function_output.tokens.unwrap_or_default().unpack_first()?;

        save_proposal_action_in_cache(
            proposal_id as i32,
            ProposalActionType::Queued(timestamp_block, execution_time as i64),
        )
    }

    Ok(())
}
