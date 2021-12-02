use anyhow::Context;
use nekoton_abi::*;
use nekoton_utils::repack_address;
use sqlx::types::Decimal;
use ton_abi::{TokenValue, Uint};
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
    // get expected proposal address
    let dao_address = repack_address(DAO_ROOT_ADDRESS).unwrap();
    let function_output = node
        .run_local(
            &dao_address,
            &expected_proposal_address(),
            &[
                answer_id(),
                ton_abi::Token::new(
                    "proposalId",
                    TokenValue::Uint(Uint::new(data.proposal_id as u128, 32)),
                ),
            ],
        )
        .await?
        .context("none function output")?;
    let proposal_address: ExpectedProposalAddress =
        function_output.tokens.unwrap_or_default().unpack()?;

    // get  proposal overview
    let function_output = node
        .run_local(&proposal_address.value0, &get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalOverview = function_output.tokens.unwrap_or_default().unpack()?;

    // get proposal config
    let function_output = node
        .run_local(&proposal_address.value0, &get_config(), &[answer_id()])
        .await?
        .context("none function output")?;
    let config: GetProposalConfig = function_output.tokens.unwrap_or_default().unpack()?;

    let payload = CreateProposal::new(
        timestamp_block,
        message_hash,
        transaction_hash,
        data.proposal_id,
        proposal,
        data.eth_actions,
        data.ton_actions,
        config.value0.grace_period,
        proposal_address.value0,
    );

    sqlx_client.create_proposal(payload).await?;

    Ok(())
}

pub async fn parse_vote_cast_event(
    vote: VoteCast,
    timestamp_block: i32,
    message_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
    user_data_address: MsgAddressInt,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    // get userdata details
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
    let id = vote.proposal_id;

    let payload = CreateVote::new(
        timestamp_block,
        message_hash,
        transaction_hash,
        vote,
        user_data_address,
    );

    sqlx_client.create_vote(payload).await?;

    // get proposal address
    let function_output = node
        .run_local(
            &dao_root,
            &expected_proposal_address(),
            &[
                answer_id(),
                ton_abi::Token::new("proposalId", TokenValue::Uint(Uint::new(id as u128, 32))),
            ],
        )
        .await?
        .context("none function output")?;
    let details: ExpectedProposalAddress = function_output.tokens.unwrap_or_default().unpack()?;

    // get proposal overview
    let function_output = node
        .run_local(&details.value0, &get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalOverview = function_output.tokens.unwrap_or_default().unpack()?;

    let payload = UpdateProposalVotes {
        for_votes: Decimal::from(proposal.for_votes),
        against_votes: Decimal::from(proposal.against_votes),
    };

    sqlx_client
        .update_proposal_votes(payload, id as i32)
        .await?;

    Ok(())
}

pub async fn parse_proposal_executed_event(
    proposal_address: MsgAddressInt,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    // get proposal id
    let function_output = node
        .run_local(&proposal_address, &get_id(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalId = function_output.tokens.unwrap_or_default().unpack()?;

    sqlx_client
        .update_proposal_executed(proposal.id as i32)
        .await?;

    Ok(())
}

pub async fn parse_proposal_canceled_event(
    proposal_address: MsgAddressInt,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    // get proposal id
    let function_output = node
        .run_local(&proposal_address, &get_id(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalId = function_output.tokens.unwrap_or_default().unpack()?;

    sqlx_client
        .update_proposal_canceled(proposal.id as i32)
        .await?;

    Ok(())
}

pub async fn parse_proposal_queued_event(
    data: ProposalQueued,
    proposal_address: MsgAddressInt,
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    // get proposal id
    let function_output = node
        .run_local(&proposal_address, &get_id(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal: ProposalId = function_output.tokens.unwrap_or_default().unpack()?;

    sqlx_client
        .update_proposal_queued(data.execution_time as i64, proposal.id as i32)
        .await?;

    Ok(())
}
