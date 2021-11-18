use std::sync::Arc;

use dexpa::net::futures::Stream;
use futures::StreamExt;
use indexer_lib::{
    AnyExtractable, AnyExtractableOutput, ExtractInput, ParsedOutput, TransactionExt,
};
use nekoton::transport::models::RawTransaction;
use ton_block::{Deserializable, Transaction};
use ton_consumer::{ProducedTransaction, TransactionProducer};
use ton_types::UInt256;

use crate::indexer::extract_events::{
    extract_ethereum_event_configuration_parsed_events, extract_staking_parsed_events,
    extract_token_transfer_ethereum_event_parsed_events,
    extract_token_transfer_ton_even_parsed_events, extract_ton_event_configuration_parsed_events,
    extract_user_data_parsed_events,
};
use crate::indexer::utils::from_transaction_to_raws_transactions;
use crate::models::events::AllEvents;
use crate::sqlx_client::SqlxClient;

pub mod abi;
mod extract_events;
mod parse_staking_events;
mod parse_transfer_events;
pub mod utils;

pub async fn bridge_dao_indexer(
    sqlx_client: SqlxClient,
    transaction_producer: Arc<TransactionProducer>,
    mut stream_transactions: impl Stream<Item = ProducedTransaction> + std::marker::Unpin,
    all_events: AllEvents,
) {
    log::debug!("start transaction_producer loop");

    loop {
        let events_prep = all_events.get_all_events();

        if sqlx_client.count_transaction().await == 0 {
            parse_history(sqlx_client.clone(), &all_events, &transaction_producer).await;
        }

        while let Some(mut produced_transaction) = stream_transactions.next().await {
            let transaction: Transaction = produced_transaction.transaction.clone();

            if extract_events(&transaction, transaction.tx_hash().unwrap(), &events_prep).is_some()
            {
                let (raw_transaction, raw_transaction_from_db) =
                    from_transaction_to_raws_transactions(transaction);

                sqlx_client
                    .new_raw_transaction(raw_transaction_from_db.clone())
                    .await;

                if let Err(e) = parse_new_event(
                    raw_transaction,
                    sqlx_client.clone(),
                    &all_events,
                    &transaction_producer,
                )
                .await
                {
                    log::error!("{}", e);
                }
                produced_transaction.commit().unwrap();
            }
        }
    }
}

pub async fn parse_new_event(
    raw_transaction: RawTransaction,
    sqlx_client: SqlxClient,
    all_events: &AllEvents,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.staking,
    ) {
        if let Err(e) = extract_staking_parsed_events(sqlx_client.clone(), node, events).await {
            log::error!("{}", e);
        }
    }

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.user_data,
    ) {
        if let Err(e) = extract_user_data_parsed_events(sqlx_client.clone(), node, events).await {
            log::error!("{}", e);
        }
    }

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.ethereum_event_configuration,
    ) {
        if let Err(e) =
            extract_ethereum_event_configuration_parsed_events(sqlx_client.clone(), node, events)
                .await
        {
            log::error!("{}", e);
        }
    };

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.ton_event_configuration,
    ) {
        if let Err(e) =
            extract_ton_event_configuration_parsed_events(sqlx_client.clone(), node, events).await
        {
            log::error!("{}", e);
        }
    };

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.token_transfer_ton_even,
    ) {
        if let Err(e) =
            extract_token_transfer_ton_even_parsed_events(sqlx_client.clone(), node, events).await
        {
            log::error!("{}", e);
        }
    };

    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.token_transfer_ethereum_event,
    ) {
        if let Err(e) =
            extract_token_transfer_ethereum_event_parsed_events(sqlx_client.clone(), node, events)
                .await
        {
            log::error!("{}", e);
        }
    };

    Ok(())
}

pub async fn parse_history(
    sqlx_client: SqlxClient,
    events_prep: &AllEvents,
    node: &TransactionProducer,
) {
    let mut raw_count = sqlx_client.count_raw_transactions().await;
    let mut offset = 0;
    while raw_count > 0 {
        let raw_transactions_from_db = sqlx_client
            .get_raw_transactions(5000, offset)
            .await
            .map_err(|e| {
                log::error!("{}", e);
                e
            })
            .unwrap_or_default();
        offset += 5000;
        raw_count -= 5000;
        for raw_transaction_from_db in raw_transactions_from_db {
            let transaction = match ton_block::Transaction::construct_from_bytes(
                raw_transaction_from_db.transaction.as_slice(),
            ) {
                Ok(a) => a,
                Err(e) => {
                    log::error!("Failed constructing tx from db: {}", e);
                    continue;
                }
            };

            let raw_transaction = RawTransaction {
                hash: UInt256::from_be_bytes(&raw_transaction_from_db.transaction_hash),
                data: transaction,
            };

            if let Err(e) =
                parse_new_event(raw_transaction, sqlx_client.clone(), events_prep, node).await
            {
                log::error!("{}", e);
            }
        }
    }
}

pub fn extract_events(
    data: &Transaction,
    hash: UInt256,
    events: &[AnyExtractable],
) -> Option<ParsedOutput<AnyExtractableOutput>> {
    ExtractInput {
        transaction: data,
        what_to_extract: events,
        hash,
    }
    .process()
    .map_err(|e| log::error!("Failed parsing: {}", e))
    .ok()
    .flatten()
}
