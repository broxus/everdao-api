use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use futures::prelude::*;
use sqlx::postgres::PgPoolOptions;
use transaction_buffer::models::{BufferedConsumerChannels, BufferedConsumerConfig};
use transaction_buffer::start_parsing_and_get_channels;
use transaction_consumer::{ConsumerOptions, TransactionConsumer};

use crate::api::*;
use crate::indexer::*;
use crate::models::AllEvents;
use crate::services::*;
use crate::settings::*;
use crate::sqlx_client::*;

pub async fn start_server() -> Result<()> {
    let config = Arc::new(get_config());
    stackdriver_logger::init_with_cargo!();

    std::panic::set_hook(Box::new(handle_panic));
    let _guard = sentry::init(
        sentry::ClientOptions::default().add_integration(sentry_panic::PanicIntegration::default()),
    );
    tokio::spawn(healthcheck_service(config.healthcheck_addr));

    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_size)
        .connect(&config.database_url)
        .await
        .expect("fail pg pool");

    sqlx::migrate!().run(&pool).await?;

    let sqlx_client = SqlxClient::new(pool.clone());

    // kafka connection
    let (group_id, topic, states_rpc_endpoint, options) = get_kafka_settings(&config);
    let transaction_consumer = TransactionConsumer::new(
        &group_id,
        &topic,
        vec![states_rpc_endpoint],
        None,
        ConsumerOptions {
            kafka_options: options
                .iter()
                .map(|(x, y)| (x.as_str(), y.as_str()))
                .collect::<HashMap<_, _>>(),
            skip_0_partition: true,
        },
    )
    .await
    .expect("Failed to get transaction producer");

    let BufferedConsumerChannels {
        rx_parsed_events,
        tx_commit,
        notify_for_services,
    } = start_parsing_and_get_channels(BufferedConsumerConfig {
        delay: 15,
        transaction_consumer: transaction_consumer.clone(),
        pg_pool: pool,
        events_to_parse: AllEvents::new().get_all_events().any_extractable,
    });

    {
        let sqlx_client = sqlx_client.clone();
        let transaction_consumer = transaction_consumer.clone();
        tokio::spawn(bridge_dao_indexer(
            sqlx_client,
            transaction_consumer,
            rx_parsed_events,
            tx_commit,
        ));
    }

    {
        let sqlx_client = sqlx_client.clone();
        tokio::spawn(fail_transaction_monitor(sqlx_client, transaction_consumer));
    }

    log::debug!("start http server");
    let service = Arc::new(Services::new(sqlx_client.clone()));

    let prod_url = config.indexer_prod_url.clone();
    let test_url = config.indexer_test_url.clone();
    notify_for_services.notified().await;
    log::info!("start http service");
    tokio::spawn(http_service(
        config.server_addr,
        service,
        sqlx_client,
        prod_url,
        test_url,
    ));

    future::pending().await
}

fn get_config() -> Config {
    Config::new().unwrap_or_else(|e| panic!("Error parsing config: {}", e))
}

fn get_kafka_settings(config: &Config) -> (String, String, String, HashMap<String, String>) {
    let mut kafka_settings: HashMap<String, String> = Default::default();
    kafka_settings.insert("bootstrap.servers".into(), config.brokers.clone());
    kafka_settings.insert("client.id".into(), config.kafka_client_id.clone());

    (
        config.kafka_group_id.clone(),      // group_id
        config.kafka_topic.clone(),         // topic
        config.states_rpc_endpoint.clone(), // states_rpc_endpoint
        kafka_settings,
    )
}

async fn healthcheck_service<A: tokio::net::ToSocketAddrs>(addr: A) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    loop {
        listener.accept().await.unwrap();
    }
}

fn handle_panic(panic_info: &std::panic::PanicInfo<'_>) {
    log::error!("{:?}", panic_info);
    std::process::exit(1);
}
