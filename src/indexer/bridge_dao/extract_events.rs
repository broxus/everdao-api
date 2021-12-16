use indexer_lib::{split, AnyExtractableOutput, ParsedOutput};
use nekoton_abi::{UnpackAbiPlain, UnpackFirst};
use ton_consumer::TransactionProducer;

use super::parse_dao_events::*;
use super::parse_proposal_events::*;
use super::parse_userdata_events::*;
use crate::models::*;
use crate::sqlx_client::*;

pub async fn extract_dao_root_parsed_events(
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = &events.transaction;

    let (_, events) = split(events.output);
    for event in events {
        let message_hash = event.message_hash.to_vec();
        if event.function_name.as_str() == "ProposalCreated" {
            let _data: ProposalCreatedTest1 = event.input.clone().unpack()?;
            log::info!("ProposalCreatedTest1 unpacked");
            let _data: ProposalCreatedTest2 = event.input.clone().unpack()?;
            log::info!("ProposalCreatedTest2 unpacked");
            let _data: ProposalCreatedTest3 = event.input.clone().unpack()?;
            log::info!("ProposalCreatedTest3 unpacked");
            let data: ProposalCreated = event.input.unpack()?;
            log::info!("ProposalCreated unpacked");
            parse_proposal_created_event(
                data,
                message_hash,
                transaction,
                sqlx_client,
                transaction_producer,
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn extract_proposal_parsed_events(
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = &events.transaction;

    let (_, events) = split(events.output);
    for event in events {
        match event.function_name.as_str() {
            "Executed" => {
                parse_proposal_executed_event(transaction, sqlx_client, node).await?;
            }
            "Canceled" => {
                parse_proposal_canceled_event(transaction, sqlx_client, node).await?;
            }
            "Queued" => {
                let execution_time: u32 = event.input.unpack_first()?;
                parse_proposal_queued_event(execution_time, transaction, sqlx_client, node).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn extract_userdata_parsed_events(
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = &events.transaction;

    let (_, events) = split(events.output);
    for event in events {
        let message_hash = event.message_hash.to_vec();
        if event.function_name.as_str() == "VoteCast" {
            let vote: VoteCast = event.input.unpack()?;
            parse_vote_cast_event(
                vote,
                message_hash,
                transaction,
                sqlx_client,
                transaction_producer,
            )
            .await?;
        }
    }
    Ok(())
}
