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

pub async fn parse_proposal_created_event(
    data: ProposalCreated,
    message_hash: Vec<u8>,
    transaction: &Transaction,
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    log::debug!("Found new proposal : {:?}", data);
    log::debug!(
        "transaction DAO ROOT address: {}",
        transaction.contract_address()?.address().to_hex_string()
    );
    if transaction.contract_address()? != repack_address(super::DAO_ROOT_ADDRESS)? {
        // skip event
        return Ok(());
    }

    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    // get expected proposal address
    let dao_root_address = transaction.contract_address()?;
    let function_output = transaction_producer
        .run_local(
            &dao_root_address,
            expected_proposal_address(),
            &[
                answer_id(),
                data.proposal_id.token_value().named("proposalId"),
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

    // get proposal config
    let function_output = transaction_producer
        .run_local(&proposal_address, get_config(), &[answer_id()])
        .await?
        .context("none function output")?;
    let proposal_config: ProposalConfig =
        function_output.tokens.unwrap_or_default().unpack_first()?;

    let ton_actions: Result<Vec<_>, _> = data
        .ton_actions
        .into_iter()
        .map(TryFrom::try_from)
        .collect();

    let proposal = CreateProposal {
        id: data.proposal_id as i32,
        address: proposal_address.to_string(),
        proposer: proposal_overview.proposer.to_string(),
        description: proposal_overview.description,
        start_time: proposal_overview.start_time as i64,
        end_time: proposal_overview.end_time as i64,
        execution_time: proposal_overview.execution_time as i64,
        grace_period: proposal_config.grace_period as i64,
        time_lock: proposal_config.time_lock as i64,
        voting_delay: proposal_config.voting_delay as i64,
        for_votes: Decimal::from(proposal_overview.for_votes),
        against_votes: Decimal::from(proposal_overview.against_votes),
        quorum_votes: Decimal::from(proposal_overview.quorum_votes),
        message_hash,
        transaction_hash,
        timestamp_block,
        actions: ProposalActions {
            ton_actions: ton_actions?,
            eth_actions: data.eth_actions.into_iter().map(From::from).collect(),
        },
    };

    sqlx_client.create_proposal(proposal).await?;

    Ok(())
}
