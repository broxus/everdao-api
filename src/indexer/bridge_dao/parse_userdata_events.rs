use anyhow::Context;
use indexer_lib::TransactionExt;
use nekoton_abi::*;
use nekoton_utils::{repack_address, TrustMe};
use sqlx::types::Decimal;
use ton_block::Transaction;
use ton_consumer::TransactionProducer;

use crate::global_cache::*;
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
        voter: details.user.to_string(),
        support: vote.support,
        reason: vote.reason,
        votes: Decimal::from(vote.votes),
        message_hash,
        transaction_hash,
        timestamp_block,
    };

    sqlx_client.create_vote(payload).await?;

    let unlock_vote = UnlockVote {
        proposal_id: vote.proposal_id as i32,
        voter: details.user.to_string(),
    };
    if remove_vote_actions_from_cache(unlock_vote.clone()) {
        sqlx_client.unlock_vote(unlock_vote).await?;
    }

    let payload = if vote.support {
        UpdateProposalVotes {
            for_votes: Decimal::from(vote.votes),
            against_votes: Decimal::ZERO,
        }
    } else {
        UpdateProposalVotes {
            for_votes: Decimal::ZERO,
            against_votes: Decimal::from(vote.votes),
        }
    };

    if sqlx_client
        .update_proposal_votes(vote.proposal_id as i32, payload.clone())
        .await?
        == 0
    {
        save_proposal_action_in_cache(
            vote.proposal_id as i32 as i32,
            ProposalActionType::Vote(payload),
        )
    }

    Ok(())
}

pub async fn parse_unlock_casted_votes_event(
    proposal_id: u32,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found unlock casted votes event");

    // get userdata details
    let user_data_address = transaction.contract_address()?;
    let function_output = transaction_producer
        .run_local(&user_data_address, get_user_data_details(), &[answer_id()])
        .await?
        .context("none function output")?;
    let details: GetDetails = function_output.tokens.unwrap_or_default().unpack_first()?;

    let vote = UnlockVote {
        proposal_id: proposal_id as i32,
        voter: details.user.to_string(),
    };

    log::debug!("Unlock event details {:?}", vote);

    if sqlx_client.unlock_vote(vote.clone()).await? == 0 {
        save_locked_vote_in_cache(vote)?;
    }

    Ok(())
}
