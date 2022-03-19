use anyhow::Context;
use indexer_lib::TransactionExt;
use nekoton_abi::*;
use ton_block::{MsgAddressInt, Transaction};
use transaction_consumer::TransactionConsumer;

use crate::global_cache::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_proposal_executed_event(
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_consumer: &TransactionConsumer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found proposal executed event");

    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    let function_output = transaction_consumer
        .run_local(&proposal_address, get_dao_root(), &[])
        .await?
        .context("none function output")?;
    let dao_root_address: MsgAddressInt =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    if dao_root_address != *super::DAO_ROOT_ADDRESS {
        // skip event
        return Ok(());
    }

    if sqlx_client
        .update_proposal_executed(proposal_address.to_string(), timestamp_block)
        .await
        .is_err()
    {
        save_proposal_action_in_cache(
            proposal_address,
            ProposalActionType::Executed(timestamp_block),
        )
    }

    Ok(())
}

pub async fn parse_proposal_canceled_event(
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_consumer: &TransactionConsumer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found proposal canceled event");

    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    let function_output = transaction_consumer
        .run_local(&proposal_address, get_dao_root(), &[])
        .await?
        .context("none function output")?;
    let dao_root_address: MsgAddressInt =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    if dao_root_address != *super::DAO_ROOT_ADDRESS {
        // skip event
        return Ok(());
    }

    if sqlx_client
        .update_proposal_canceled(proposal_address.to_string(), timestamp_block)
        .await
        .is_err()
    {
        save_proposal_action_in_cache(
            proposal_address,
            ProposalActionType::Canceled(timestamp_block),
        )
    }

    Ok(())
}

pub async fn parse_proposal_queued_event(
    execution_time: u32,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_consumer: &TransactionConsumer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found proposal queued event");
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    let function_output = transaction_consumer
        .run_local(&proposal_address, get_dao_root(), &[])
        .await?
        .context("none function output")?;
    let dao_root_address: MsgAddressInt =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    if dao_root_address != *super::DAO_ROOT_ADDRESS {
        // skip event
        return Ok(());
    }

    if sqlx_client
        .update_proposal_queued(
            proposal_address.to_string(),
            timestamp_block,
            execution_time as i64,
        )
        .await
        .is_err()
    {
        save_proposal_action_in_cache(
            proposal_address,
            ProposalActionType::Queued(timestamp_block, execution_time as i64),
        )
    }

    Ok(())
}
