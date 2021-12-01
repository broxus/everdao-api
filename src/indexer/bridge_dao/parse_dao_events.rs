use anyhow::Context;
use nekoton_abi::*;
use ton_block::MsgAddressInt;
use ton_consumer::TransactionProducer;

use crate::models::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_proposal_created_event(
    data: ProposalCreated,
    timestamp_block: i32,
    message_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
    dao_address: MsgAddressInt,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let function_output = node
        .run_local(
            &dao_address,
            &expected_proposal_address(),
            &[answer_id(), data.proposal_id],
        )
        .await?
        .context("none function output")?;
    let details: ExpectedProposalAddress = function_output.tokens.unwrap_or_default().unpack()?;

    let function_output = node
        .run_local(&details.value0, &get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalOverview = function_output.tokens.unwrap_or_default().unpack()?;

    Ok(())
}

pub async fn parse_vote_cast_event(
    data: VoteCast,
    timestamp_block: i32,
    message_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
    user_data_address: MsgAddressInt,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {

    let function_output = node
        .run_local(
            &user_data_address,
            &get_user_data_details(),
            &[answer_id()],
        )
        .await?
        .context("none function output")?;
    let details: GetDetails = function_output.tokens.unwrap_or_default().unpack()?;

    let dao_root = details.value0.dao_root;

    let function_output = node
        .run_local(
            &dao_root,
            &expected_proposal_address(),
            &[answer_id(), data.proposal_id],
        )
        .await?
        .context("none function output")?;
    let details: ExpectedProposalAddress = function_output.tokens.unwrap_or_default().unpack()?;

    let function_output = node
        .run_local(&details.value0, &get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalOverview = function_output.tokens.unwrap_or_default().unpack()?;



    Ok(())
}


