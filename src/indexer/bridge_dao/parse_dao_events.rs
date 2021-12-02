use anyhow::Context;
use nekoton_abi::*;
use nekoton_utils::repack_address;
use ton_block::MsgAddressInt;
use ton_consumer::TransactionProducer;

use crate::models::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

const DAO_ROOT_ADDRESS: &str = "0:3c33153078ea2b94144ad058812563f4896cadbb84e7cc55c08e24e0a394fb3e";

pub async fn parse_proposal_created_event(
    data: ProposalCreated,
    timestamp_block: i32,
    message_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let dao_address = repack_address(DAO_ROOT_ADDRESS).unwrap();
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

    let eth_actions = data.eth_actions;
    let ton_actions = data.ton_actions;

    // get proposal config - grace_period?

    let payload = CreateProposal::new(
        timestamp_block,
        message_hash,
        transaction_hash,
        data.proposal_id,
        proposal,
        eth_actions,
        ton_actions,
        grace_period,
    );

    sqlx_client.create_proposal(payload).await?;

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
        .run_local(&user_data_address, &get_user_data_details(), &[answer_id()])
        .await?
        .context("none function output")?;
    let details: GetDetails = function_output.tokens.unwrap_or_default().unpack()?;

    let dao_root = details.value0.dao_root;

    let dao_address = repack_address(DAO_ROOT_ADDRESS).unwrap();

    if dao_root != dao_address {
        log::error!(
            "Wrong dao root address in vote cast user data - {}",
            dao_root.address()
        );
        return Ok(());
    }

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
