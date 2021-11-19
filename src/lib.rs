#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::inconsistent_struct_constructor)]
#![deny(clippy::dbg_macro)]

use std::sync::Arc;

use chrono::Utc;
use dexpa::prelude::StdResult;
use dexpa::utils::handle_panic;
use futures::prelude::*;
use indexer_lib::TransactionExt;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use ton_block::Transaction;
use ton_consumer::TransactionProducer;

use crate::api::http_service;
use crate::indexer::utils::{from_transaction_to_raws_transactions, update_frozen_staking};
use crate::indexer::{bridge_dao_indexer, extract_events, parse_history};
use crate::models::events::AllEvents;
use crate::reqwest_client::{loop_update_vault_info, update_vault_info, ReqwestClient};
use crate::services::Services;
use crate::settings::Config;
use crate::sqlx_client::SqlxClient;

pub mod api;
pub mod indexer;
pub mod models;
mod services;
mod settings;
mod sqlx_client;

pub async fn start_server() -> StdResult<()> {
    let config = get_config();
    let config = Arc::new(config);
    stackdriver_logger::init_with_cargo!();

    std::panic::set_hook(Box::new(handle_panic));
    let _guard = sentry::init(
        sentry::ClientOptions::default().add_integration(sentry_panic::PanicIntegration::default()),
    );
    tokio::spawn(dexpa::net::healthcheck_service(config.healthcheck_addr));

    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(&config.database_url)
        .await
        .expect("fail pg pool");
    let sqlx_client = SqlxClient::new(pool);

    // kafka connection
    let (group_id, topic, states_rpc_endpoint, options) = get_kafka_settings(&config);
    let transaction_producer = TransactionProducer::new(
        &group_id,
        topic,
        states_rpc_endpoint,
        options
            .iter()
            .map(|(x, y)| (x.as_str(), y.as_str()))
            .collect::<HashMap<_, _>>(),
    )
    .expect("fail get transaction producer");

    let mut stream_transactions = transaction_producer.clone().stream_blocks().await.unwrap();
    let all_events = AllEvents::new();
    let prep_events = all_events.get_all_events();

    if sqlx_client.count_transaction().await == 0 {
        parse_history(sqlx_client.clone(), &all_events, &transaction_producer).await;
        if sqlx_client.count_transaction().await == 0 {
            log::info!("start get history transactions");
            let timestamp_now = Utc::now().timestamp() as i32;

            while let Some(mut produced_transaction) = stream_transactions.next().await {
                let transaction: Transaction = produced_transaction.transaction.clone();
                if extract_events(&transaction, transaction.tx_hash().unwrap(), &prep_events)
                    .is_some()
                {
                    let (_raw_transaction, raw_transaction_from_db) =
                        from_transaction_to_raws_transactions(transaction.clone());
                    sqlx_client
                        .new_raw_transaction(raw_transaction_from_db)
                        .await;
                }
                if transaction.time() as i32 - timestamp_now > 60 * 60 {
                    produced_transaction.commit().unwrap();
                    break;
                }
            }
            log::info!("start parse history");
            parse_history(sqlx_client.clone(), &all_events, &transaction_producer).await;
            log::info!("end history");
        }
    }

    {
        let sqlx_client = sqlx_client.clone();
        tokio::spawn(update_frozen_staking(sqlx_client));
    }

    {
        let sqlx_client = sqlx_client.clone();
        tokio::spawn(bridge_dao_indexer(
            sqlx_client,
            transaction_producer,
            stream_transactions,
            all_events,
        ));
    }

    let service = Arc::new(Services::new(&config, sqlx_client.clone()));
    tokio::spawn(http_service(config.server_addr, service, sqlx_client));

    future::pending().await
}

fn get_config() -> Config {
    settings::Config::new().unwrap_or_else(|e| panic!("Error parsing config: {}", e))
}

fn get_kafka_settings(config: &Config) -> (String, String, String, HashMap<String, String>) {
    use std::fs::File;
    use std::io::Read;

    let mut file =
        File::open(config.kafka_settings_path.clone()).expect("can't find kafka settings");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("can't read from kafka file");

    let mut kafka_settings: HashMap<String, String> =
        serde_json::from_str(&contents).expect("can not parse kafka from config");
    kafka_settings.insert("client.id".into(), config.kafka_client_id.clone());
    kafka_settings.insert("sasl.username".into(), config.kafka_user.clone());
    kafka_settings.insert("sasl.password".into(), config.kafka_password.clone());

    (
        config.kafka_group_id.clone(),      // group_id
        "ton-transactions-1".into(),        // topic
        config.states_rpc_endpoint.clone(), // states_rpc_endpoint
        kafka_settings,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Local;
    use env_logger::init;
    use env_logger::Builder;
    use log::LevelFilter;
    use sqlx::PgPool;
    use std::collections::HashMap;
    use std::io::Write;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_history() {
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} {}{} {} [{}] - {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.module_path().unwrap_or_default(),
                    record.file().unwrap_or_default(),
                    record.line().unwrap_or_default(),
                    record.level(),
                    record.args(),
                )
            })
            .filter(None, LevelFilter::Error)
            .init();
        let pg_pool =
            PgPool::connect("postgresql://postgres:postgres@localhost:5432/bridge_dao_indexer")
                .await
                .unwrap();
        let sqlx_client = SqlxClient::new(pg_pool);
        // let graphql_client = EVMGraphqlClient::default();
        let reqwest_client = ReqwestClient::new("https://ton-swap-indexer.broxus.com".to_string());
        // update_vault_info(
        //     graphql_client.clone(),
        //     sqlx_client.clone(),
        //     reqwest_client.clone(),
        // )
        // .await;

        let transaction_producer = TransactionProducer::new(
            "ton-bridge-local-test-0",
            "ton-transactions-1".into(),
            "http://35.240.13.113:8081",
            HashMap::from([
                ("client.id", "ton-test"),
                ("enable.auto.commit", "false"),
                ("auto.offset.reset", "earliest"),
                (
                    "bootstrap.servers",
                    "kafka1.dexpa.io:9093, kafka2.dexpa.io:9093, kafka3.dexpa.io:9093",
                ),
                ("security.protocol", "SASL_SSL"),
                ("sasl.mechanism", "SCRAM-SHA-512"),
                ("ssl.ca.location", "kafka_client.pem"),
                ("sasl.username", "ton-reader"),
                ("sasl.password", ""),
            ]),
        )
        .unwrap();
        log::error!("start get history transactions");
        let mut stream_transactions = transaction_producer.clone().stream_blocks().await.unwrap();
        let all_events = AllEvents::new();
        let prep_events = all_events.get_all_events();
        let timestamp_now = Utc::now().timestamp() as i32;

        // while let Some(mut produced_transaction) = stream_transactions.next().await {
        //     let transaction: Transaction = produced_transaction.transaction.clone();
        //     if extract_events(&transaction, transaction.tx_hash().unwrap(), &prep_events).is_some()
        //     {
        //         let (_raw_transaction, raw_transaction_from_db) =
        //             from_transaction_to_raws_transactions(transaction.clone());
        //         sqlx_client
        //             .new_raw_transaction(raw_transaction_from_db)
        //             .await;
        //     }
        //     if transaction.time() as i32 - timestamp_now > 60 * 90 {
        //         produced_transaction.commit().unwrap();
        //         break;
        //     }
        // }
        log::error!("start parse history");
        parse_history(sqlx_client.clone(), &all_events, &transaction_producer).await;
        log::error!("end history");
    }
}
