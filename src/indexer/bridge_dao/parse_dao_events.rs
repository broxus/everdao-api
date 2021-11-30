use anyhow::Context;
use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_consumer::TransactionProducer;

use crate::models::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_proposal_created_event(
    data: ProposalCreated,
    _timestamp_block: i32,
    _message_hash: Vec<u8>,
    _transaction_hash: Vec<u8>,
    dao_address: MsgAddressInt,
    _sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {

    let function_output = node
        .run_local(&dao_address, &expected_proposal_address(), &[answer_id(), data.proposal_id])
        .await?
        .context("none function output")?;
    let details: ExpectedProposalAddress = function_output.tokens.unwrap_or_default().unpack()?;

    let function_output = node
        .run_local(&details.value0, &get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let _details: ProposalOverview = function_output.tokens.unwrap_or_default().unpack()?;

    Ok(())
}

pub async fn parse_vote_cast_event(
    data: VoteCast,
    _timestamp_block: i32,
    _message_hash: Vec<u8>,
    _transaction_hash: Vec<u8>,
    _sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {

    Ok(())
}
