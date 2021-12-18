use std::sync::Arc;

use dexpa::net::futures::Stream;
use futures::StreamExt;
use indexer_lib::{split, AnyExtractableOutput, ExtractInput, ParsedOutput, TransactionExt};
use nekoton::transport::models::RawTransaction;
use nekoton_utils::TrustMe;
use ton_block::{GetRepresentationHash, Transaction};
use ton_consumer::{ProducedTransaction, TransactionProducer};
use ton_types::UInt256;

use crate::models::*;
use crate::sqlx_client::*;

use self::extract_events::*;

mod extract_events;
mod parse_dao_events;
mod parse_proposal_events;
mod parse_userdata_events;

const DAO_ROOT_ADDRESS: &str = "0:3c33153078ea2b94144ad058812563f4896cadbb84e7cc55c08e24e0a394fb3e";

pub async fn bridge_dao_indexer(
    sqlx_client: SqlxClient,
    transaction_producer: Arc<TransactionProducer>,
    mut stream_transactions: impl Stream<Item = ProducedTransaction> + std::marker::Unpin,
) {
    log::info!("Start Bridge-Dao indexer...");

    let all_events = AllEvents::new();
    let prep_events = all_events.get_all_events();

    while let Some(produced_transaction) = stream_transactions.next().await {
        let transaction = produced_transaction.transaction.clone();

        if extract_events(&transaction, transaction.tx_hash().trust_me(), &prep_events).is_some() {
            let (raw_transaction, raw_transaction_from_db) =
                from_transaction_to_raws_transactions(transaction);

            if let Err(err) = sqlx_client
                .create_raw_transaction(raw_transaction_from_db)
                .await
            {
                log::error!("Failed to create raw transaction in db: {}", err);
            }

            if let Err(err) = parse_new_event(
                raw_transaction,
                &sqlx_client,
                &all_events,
                &transaction_producer,
            )
            .await
            {
                log::error!("Failed to parse event: {}", err);
            }

            // produced_transaction.commit().trust_me();
        }
    }

    panic!("rip kafka consumer");
}

fn extract_events(
    data: &Transaction,
    hash: UInt256,
    events_parsing: &EventsParsing,
) -> Option<ParsedOutput<AnyExtractableOutput>> {
    ExtractInput {
        transaction: data,
        what_to_extract: &events_parsing.any_extractable,
        hash,
    }
    .process()
    .map_err(|e| log::error!("Failed parsing: {:?}", e))
    .ok()
    .flatten()
    .filter(|x| {
        let (parsed_functions, parsed_events) = split(x.output.clone());
        for function in parsed_functions {
            if events_parsing
                .functions_check
                .contains(&(function.function_name, function.function_id))
            {
                return true;
            }
        }
        for event in parsed_events {
            if events_parsing
                .events_check
                .contains(&(event.function_name, event.event_id))
            {
                return true;
            }
        }
        false
    })
}

async fn parse_new_event(
    raw_transaction: RawTransaction,
    sqlx_client: &SqlxClient,
    all_events: &AllEvents,
    transaction_producer: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.dao_root,
    ) {
        if let Err(e) =
            extract_dao_root_parsed_events(sqlx_client, transaction_producer, events).await
        {
            log::error!("{}", e);
        }
    }

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.proposal,
    ) {
        if let Err(e) =
            extract_proposal_parsed_events(sqlx_client, transaction_producer, events).await
        {
            log::error!("{}", e);
        }
    }

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.user_data,
    ) {
        if let Err(e) =
            extract_userdata_parsed_events(sqlx_client, transaction_producer, events).await
        {
            log::error!("{}", e);
        }
    }

    Ok(())
}

fn from_transaction_to_raws_transactions(
    transaction: Transaction,
) -> (RawTransaction, RawTransactionFromDb) {
    let raw_transaction = RawTransaction {
        hash: transaction.hash().trust_me(),
        data: transaction,
    };

    let raw_transaction_from_db: RawTransactionFromDb =
        raw_transaction.clone().try_into().trust_me();

    (raw_transaction, raw_transaction_from_db)
}
