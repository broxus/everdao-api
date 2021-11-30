use std::sync::Arc;

use dexpa::net::futures::Stream;
use futures::StreamExt;
use indexer_lib::{
    AnyExtractable, AnyExtractableOutput, ExtractInput, ParsedOutput, TransactionExt,
};
use nekoton::transport::models::RawTransaction;
use nekoton_utils::TrustMe;
use ton_block::Transaction;
use ton_consumer::{ProducedTransaction, TransactionProducer};
use ton_types::UInt256;

use crate::models::*;
use crate::sqlx_client::*;

use self::extract_events::*;

mod extract_events;
mod parse_dao_events;

pub async fn bridge_dao_indexer(
    sqlx_client: SqlxClient,
    transaction_producer: Arc<TransactionProducer>,
    mut stream_transactions: impl Stream<Item = ProducedTransaction> + std::marker::Unpin,
) {
    log::info!("Start Bridge-Dao indexer...");

    let all_events = AllEvents::new();
    let prep_events = all_events.get_all_events();

    while let Some(mut produced_transaction) = stream_transactions.next().await {
        let transaction: Transaction = produced_transaction.transaction.clone();

        if extract_events(&transaction, transaction.tx_hash().trust_me(), &prep_events).is_some() {
            match transaction.clone().try_into() {
                Ok(raw_transaction_db) => {
                    if let Err(err) = sqlx_client.create_raw_transaction(raw_transaction_db).await {
                        log::error!("Failed to create raw transaction in db: {}", err);
                    }
                }
                Err(err) => {
                    log::error!(
                        "Failed to convert Transaction to RawTransactionFromDb: {}",
                        err
                    );
                }
            };

            if let Err(err) = parse_transaction(
                RawTransaction {
                    hash: transaction.tx_hash().trust_me(),
                    data: transaction,
                },
                &sqlx_client,
                &all_events,
                &transaction_producer,
            )
            .await
            {
                log::error!("Failed to parse event: {}", err);
            }

            produced_transaction.commit().trust_me();
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

pub async fn parse_transaction(
    raw_transaction: RawTransaction,
    sqlx_client: &SqlxClient,
    all_events: &AllEvents,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    if let Some(events) = extract_events(
        &raw_transaction.data,
        raw_transaction.hash,
        &all_events.dao_root,
    ) {
        if let Err(e) = extract_dao_root_parsed_events(sqlx_client, node, events).await {
            log::error!("{}", e);
        }
    }

    Ok(())
}
