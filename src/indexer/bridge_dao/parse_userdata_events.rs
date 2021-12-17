use anyhow::Context;
use indexer_lib::TransactionExt;
use nekoton_abi::*;
use nekoton_utils::{repack_address, TrustMe};
use sqlx::types::Decimal;
use ton_block::{MsgAddressInt, Transaction};
use ton_consumer::TransactionProducer;

use crate::models::*;
use crate::sqlx_client::*;
use crate::ton_contracts::*;

pub async fn parse_vote_cast_event(
    vote: VoteCast,
    message_hash: Vec<u8>,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found vote cast event - {:?}", vote);
    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    // get userdata details
    let user_data_address = transaction.contract_address()?;
    let function_output = transaction_producer
        .run_local(&user_data_address, get_user_data_details(), &[answer_id()])
        .await?
        .context("none function output")?;
    let details: GetDetails = function_output.tokens.unwrap_or_default().unpack_first()?;

    if details.dao_root != repack_address(super::DAO_ROOT_ADDRESS)? {
        log::error!(
            "Wrong dao root address in vote cast user data - {}",
            details.dao_root.to_string()
        );
        return Ok(());
    }

    let payload = CreateVote {
        proposal_id: vote.proposal_id as i32,
        voter: user_data_address.to_string(),
        support: vote.support,
        reason: vote.reason,
        votes: Decimal::from(vote.votes),
        message_hash,
        transaction_hash,
        timestamp_block,
    };

    sqlx_client.create_vote(payload).await?;

    // get proposal address
    let function_output = transaction_producer
        .run_local(
            &details.dao_root,
            expected_proposal_address(),
            &[
                answer_id(),
                vote.proposal_id.token_value().named("proposalId"),
            ],
        )
        .await?
        .context("none function output")?;
    let proposal_address: MsgAddressInt =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    // get proposal overview
    let function_output = transaction_producer
        .run_local(&proposal_address, get_overview(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal_overview: ProposalOverview =
        function_output.tokens.unwrap_or_default().unpack()?;

    let payload = UpdateProposalVotes {
        for_votes: Decimal::from(proposal_overview.for_votes),
        against_votes: Decimal::from(proposal_overview.against_votes),
    };

    sqlx_client
        .update_proposal_votes(vote.proposal_id as i32, payload)
        .await?;

    Ok(())
}
