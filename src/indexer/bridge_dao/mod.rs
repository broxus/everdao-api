use std::sync::Arc;
use std::time::Duration;

use crate::global_cache::{is_proposal_cache_empty, is_vote_cache_empty};
use dexpa::net::futures::Stream;
use futures::StreamExt;
use indexer_lib::{split, AnyExtractableOutput, ExtractInput, ParsedOutput, TransactionExt};
use nekoton_utils::{repack_address, TrustMe};
use tokio::time;
use ton_block::{Deserializable, MsgAddressInt, Transaction};
use ton_consumer::{ProducedTransaction, TransactionProducer};
use ton_types::UInt256;

use crate::models::*;
use crate::sqlx_client::*;

use self::extract_events::*;

mod extract_events;
mod parse_dao_events;
mod parse_proposal_events;
mod parse_userdata_events;

lazy_static::lazy_static! {
    static ref DAO_ROOT_ADDRESS: MsgAddressInt =
        repack_address(&std::env::var("DAO_ROOT").trust_me()).trust_me();
}

pub async fn bridge_dao_indexer(
    sqlx_client: SqlxClient,
    transaction_producer: Arc<TransactionProducer>,
    mut stream_transactions: impl Stream<Item = ProducedTransaction> + std::marker::Unpin,
) {
    log::info!("Start Bridge-Dao indexer...");

    let all_events = AllEvents::new();
    let prep_events = all_events.get_all_events();

    while let Some(mut produced_transaction) = stream_transactions.next().await {
        let transaction = produced_transaction.transaction.clone();
        let transaction_hash = transaction.tx_hash().trust_me();

        if extract_events(&transaction, transaction_hash, &prep_events).is_some() {
            let raw_transaction_from_db: RawTransactionFromDb =
                transaction.clone().try_into().trust_me();

            if let Err(err) = sqlx_client
                .create_raw_transaction(raw_transaction_from_db)
                .await
            {
                log::error!(
                    "Failed to create a raw transaction in db: {}; Transaction hash: {}",
                    err,
                    transaction_hash.to_hex_string()
                );
            }

            match parse_new_event(
                transaction,
                transaction_hash,
                &sqlx_client,
                &all_events,
                &transaction_producer,
            )
            .await
            {
                Ok(_) => {
                    if let Err(err) = sqlx_client
                        .update_raw_transactions(
                            transaction_hash.as_slice(),
                            RawTransactionState::Success,
                        )
                        .await
                    {
                        log::error!(
                            "Failed to set transaction state to 'Success': {}; Transaction hash: {}",
                            err,
                            transaction_hash.to_hex_string()
                        );
                    }
                }
                Err(err) => {
                    log::error!(
                        "Failed to parse event: {}; Transaction hash: {}",
                        err,
                        transaction_hash.to_hex_string()
                    );
                    if let Err(err) = sqlx_client
                        .update_raw_transactions(
                            transaction_hash.as_slice(),
                            RawTransactionState::Fail,
                        )
                        .await
                    {
                        log::error!(
                            "Failed to set transaction state to 'Fail': {}; Transaction hash: {}",
                            err,
                            transaction_hash.to_hex_string()
                        );
                    }
                }
            }

            if is_proposal_cache_empty() && is_vote_cache_empty() {
                produced_transaction.commit().trust_me();
            }
        }
    }

    panic!("rip kafka consumer");
}

pub async fn fail_transaction_monitor(
    sqlx_client: SqlxClient,
    transaction_producer: Arc<TransactionProducer>,
) {
    log::info!("Start Fail Transaction Monitor...");

    let mut interval = time::interval(Duration::from_secs(300));

    let all_events = AllEvents::new();
    let prep_events = all_events.get_all_events();

    loop {
        interval.tick().await;

        let raw_transactions = match sqlx_client
            .get_raw_transactions_by_state(RawTransactionState::Fail)
            .await
        {
            Ok(raw_transactions) => raw_transactions,
            Err(err) => {
                log::error!("Failed to get raw transactions by state from db: {}", err);
                continue;
            }
        };

        let transactions = raw_transactions
            .into_iter()
            .map(|x| Transaction::construct_from_bytes(&x.transaction).trust_me())
            .collect::<Vec<Transaction>>();

        for transaction in transactions {
            let transaction_hash = transaction.tx_hash().trust_me();

            if extract_events(&transaction, transaction_hash, &prep_events).is_some() {
                if let Err(err) = parse_new_event(
                    transaction,
                    transaction_hash,
                    &sqlx_client,
                    &all_events,
                    &transaction_producer,
                )
                .await
                {
                    log::error!(
                        "Failed to parse event: {}; Transaction hash: {}",
                        err,
                        transaction_hash.to_hex_string()
                    );
                }
            }

            if let Err(err) = sqlx_client
                .update_raw_transactions(transaction_hash.as_slice(), RawTransactionState::Success)
                .await
            {
                log::error!(
                    "Failed to set transaction state to 'Success': {}; Transaction hash: {}",
                    err,
                    transaction_hash.to_hex_string()
                );
            }
        }
    }
}

fn extract_events(
    transaction: &Transaction,
    transaction_hash: UInt256,
    events_parsing: &EventsParsing,
) -> Option<ParsedOutput<AnyExtractableOutput>> {
    ExtractInput {
        transaction,
        what_to_extract: &events_parsing.any_extractable,
        hash: transaction_hash,
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
    transaction: Transaction,
    transaction_hash: UInt256,
    sqlx_client: &SqlxClient,
    all_events: &AllEvents,
    transaction_producer: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    if let Some(events) = extract_events(&transaction, transaction_hash, &all_events.dao_root) {
        if let Err(e) =
            extract_dao_root_parsed_events(sqlx_client, transaction_producer, events).await
        {
            log::error!("{}", e);
        }
    }

    if let Some(events) = extract_events(&transaction, transaction_hash, &all_events.proposal) {
        if let Err(e) =
            extract_proposal_parsed_events(sqlx_client, transaction_producer, events).await
        {
            log::error!("{}", e);
        }
    }

    if let Some(events) = extract_events(&transaction, transaction_hash, &all_events.user_data) {
        if let Err(e) =
            extract_userdata_parsed_events(sqlx_client, transaction_producer, events).await
        {
            log::error!("{}", e);
        }
    }

    Ok(())
}
