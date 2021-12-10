use anyhow::Context;
use indexer_lib::TransactionExt;
use nekoton_abi::*;
use ton_block::Transaction;
use ton_consumer::TransactionProducer;

use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_proposal_executed_event(
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    // get proposal id
    let function_output = node
        .run_local(&proposal_address, get_id(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal_id: u32 = function_output.tokens.unwrap_or_default().unpack_first()?;

    sqlx_client
        .update_proposal_executed(proposal_id as i32, timestamp_block)
        .await?;

    Ok(())
}

pub async fn parse_proposal_canceled_event(
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    // get proposal id
    let function_output = node
        .run_local(&proposal_address, get_id(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal_id: u32 = function_output.tokens.unwrap_or_default().unpack_first()?;

    sqlx_client
        .update_proposal_canceled(proposal_id as i32, timestamp_block)
        .await?;

    Ok(())
}

pub async fn parse_proposal_queued_event(
    execution_time: u32,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let timestamp_block = transaction.time() as i32;
    let proposal_address = transaction.contract_address()?;

    // get proposal id
    let function_output = node
        .run_local(&proposal_address, get_id(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal_id: u32 = function_output.tokens.unwrap_or_default().unpack_first()?;

    sqlx_client
        .update_proposal_queued(execution_time as i64, proposal_id as i32, timestamp_block)
        .await?;

    Ok(())
}
