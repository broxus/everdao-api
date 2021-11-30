use anyhow::Context;
use nekoton_abi::*;
use ton_consumer::TransactionProducer;

use crate::models::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_proposal_created_event(
    data: ProposalCreated,
    _timestamp_block: i32,
    _message_hash: Vec<u8>,
    _transaction_hash: Vec<u8>,
    _sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let function_output = node
        .run_local(&data.proposer, &get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let _details: ProposalOverview = function_output.tokens.unwrap_or_default().unpack()?;

    Ok(())
}
