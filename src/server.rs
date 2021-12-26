use std::collections::HashMap;
use std::sync::Arc;

use dexpa::prelude::StdResult;
use dexpa::utils::handle_panic;
use futures::prelude::*;
use nekoton_utils::TrustMe;
use sqlx::postgres::PgPoolOptions;
use ton_consumer::TransactionProducer;

use crate::api::http_service;
use crate::indexer::*;
use crate::services::Services;
use crate::settings::*;
use crate::sqlx_client::*;

pub async fn start_server() -> StdResult<()> {
    let config = Arc::new(get_config());
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
        &topic,
        states_rpc_endpoint,
        options
            .iter()
            .map(|(x, y)| (x.as_str(), y.as_str()))
            .collect::<HashMap<_, _>>(),
    )
    .expect("Falied to get transaction producer");

    transaction_producer.reset_offsets()?;

    let stream_transactions = transaction_producer
        .clone()
        .stream_transactions()
        .await
        .trust_me();

    {
        let sqlx_client = sqlx_client.clone();
        tokio::spawn(bridge_dao_indexer(
            sqlx_client,
            transaction_producer,
            stream_transactions,
        ));
    }

    log::debug!("start http server");
    let service = Arc::new(Services::new(sqlx_client.clone()));
    tokio::spawn(http_service(config.server_addr, service, sqlx_client));

    future::pending().await
}

fn get_config() -> Config {
    Config::new().unwrap_or_else(|e| panic!("Error parsing config: {}", e))
}

fn get_kafka_settings(config: &Config) -> (String, String, String, HashMap<String, String>) {
    use std::fs::File;
    use std::io::Read;

    let mut file =
        File::open(config.kafka_settings_path.clone()).expect("Can't find kafka settings");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("can't read from kafka file");

    let mut kafka_settings: HashMap<String, String> =
        serde_json::from_str(&contents).expect("Can't parse kafka from config");
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
