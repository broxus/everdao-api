use anyhow::Context;
use nekoton_abi::UnpackAbiPlain;
use nekoton_utils::TrustMe;
use sqlx::types::Decimal;
use ton_abi::Function;
use ton_block::MsgAddressInt;
use ton_consumer::TransactionProducer;

use crate::indexer::abi::{
    ETHEREUM_EVENT_CONFIGURATION, TOKEN_TRANSFER_ETHEREUM_EVENT, TOKEN_TRANSFER_TON_EVENT,
};
use crate::indexer::utils::build_answer_id_camel;
use crate::models::sqlx::{RelayEventFromDb, TransferFromDb};
use crate::models::transfer_enums::abi_type::AbiType;
use crate::models::transfer_enums::ton_status::TonStatusEvent;
use crate::models::transfer_enums::transfer_kind::TransferKind;
use crate::models::transfer_status::TransferStatus;
use crate::sqlx_client::SqlxClient;

pub async fn parse_ton_event_configuration_event(
    node: &TransactionProducer,
    new_contract: crate::models::abi::ton_event_configuration::NewEventContract,
    sqlx_client: SqlxClient,
    timestamp_block: i32,
    transaction_hash: Vec<u8>,
    message_hash: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let contract_address = new_contract.event_contract.to_string();
    let id = vec![build_answer_id_camel()];

    let function_output = node
        .run_local(
            &new_contract.event_contract,
            &get_ton_decoded_data_function(),
            &id,
        )
        .await?
        .context("none function output")?;
    let decoded_data: crate::models::abi::token_transfer_ton_event::GetDecodedData =
        function_output.tokens.unwrap_or_default().unpack()?;

    let function_output = node
        .run_local(
            &new_contract.event_contract,
            &get_ton_details_function(),
            &id,
        )
        .await?
        .context("none function output")?;
    let details_data: crate::models::abi::token_transfer_ton_event::GetDetails =
        function_output.tokens.unwrap_or_default().unpack()?;

    let vault_info = sqlx_client
        .get_vault_info_by_chain_id_and_proxy(
            decoded_data.chain_id as i32,
            &details_data.initializer.to_string(),
        )
        .await?;

    let mut volume_exec = Decimal::from(decoded_data.tokens);
    volume_exec
        .set_scale(vault_info.ton_currency_scale as u32)
        .unwrap();

    let transfer = TransferFromDb {
        ton_message_hash: Some(message_hash),
        ton_transaction_hash: Some(transaction_hash),
        contract_address: Some(contract_address),
        event_index: None,
        eth_transaction_hash: None,
        user_address: decoded_data.owner_address.to_string(),
        volume_exec,
        ton_token_address: vault_info.ton_token_address,
        eth_token_address: vault_info.eth_token_address,
        transfer_kind: TransferKind::TonToEth.to_string(),
        status: TransferStatus::Pending.to_string(),
        required_votes: 0,
        confirm_votes: 0,
        reject_votes: 0,
        burn_callback_timestamp_lt: Some(
            details_data.event_init_data.vote_data.event_transaction_lt as i64,
        ),
        timestamp_block_updated_at: timestamp_block,
        timestamp_block_created_at: timestamp_block,
        graphql_timestamp: None,
        chain_id: vault_info.chain_id,
        updated_at: 0,
        created_at: 0,
    };

    sqlx_client.new_ton_eth_transfer(transfer).await
}

pub async fn parse_ethereum_event_configuration_event(
    node: &TransactionProducer,
    new_contract: crate::models::abi::ethereum_event_configuration::NewEventContract,
    sqlx_client: SqlxClient,
    timestamp_block: i32,
    transaction_hash: Vec<u8>,
    message_hash: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let id = vec![build_answer_id_camel()];

    let function_output = node
        .run_local(
            &new_contract.event_contract,
            &get_eth_decoded_function(),
            &id,
        )
        .await?
        .context("none function output")?;

    let decoded_data: crate::models::abi::token_transfer_ethereum_event::GetDecodedData =
        function_output.tokens.unwrap_or_default().unpack()?;

    let function_output = node
        .run_local(
            &new_contract.event_contract,
            &get_eth_details_function(),
            &id,
        )
        .await?
        .context("none function output")?;

    let get_details_event: crate::models::abi::token_transfer_ethereum_event::GetDetails =
        function_output.tokens.unwrap_or_default().unpack()?;

    let function_output = node
        .run_local(
            &get_details_event.event_init_data.configuration,
            &get_details_eth_configuration_function(),
            &id,
        )
        .await?
        .context("none function output")?;

    let get_details_configuration: crate::models::abi::ethereum_event_configuration::GetDetails =
        function_output.tokens.unwrap_or_default().unpack()?;

    let vault_address = format!(
        "0x{}",
        hex::encode(
            get_details_configuration
                .network_configuration
                .event_emitter,
        )
    );

    let vault_info = sqlx_client
        .get_vault_info_by_vault_address(&vault_address)
        .await?;

    let mut volume_exec = Decimal::from(decoded_data.tokens);
    volume_exec
        .set_scale(vault_info.ton_currency_scale as u32)
        .unwrap();

    let transfer = TransferFromDb {
        ton_message_hash: Some(message_hash),
        ton_transaction_hash: Some(transaction_hash),
        contract_address: Some(new_contract.event_contract.to_string()),
        event_index: Some(get_details_event.event_init_data.vote_data.event_index as i32),
        eth_transaction_hash: Some(format!(
            "0x{}",
            hex::encode(
                get_details_event
                    .event_init_data
                    .vote_data
                    .event_transaction
                    .as_slice(),
            )
        )),
        user_address: decoded_data.owner_address.to_string(),
        volume_exec,
        ton_token_address: vault_info.ton_token_address,
        eth_token_address: vault_info.eth_token_address,
        transfer_kind: TransferKind::EthToTon.to_string(),
        status: TransferStatus::Pending.to_string(),
        required_votes: 0,
        confirm_votes: 0,
        reject_votes: 0,
        burn_callback_timestamp_lt: None,
        timestamp_block_updated_at: timestamp_block,
        timestamp_block_created_at: timestamp_block,
        graphql_timestamp: None,
        chain_id: vault_info.chain_id,
        updated_at: 0,
        created_at: 0,
    };

    sqlx_client.new_eth_ton_transfer(transfer.clone()).await
}

pub async fn parse_status_event(
    sqlx_client: SqlxClient,
    status: TonStatusEvent,
    contract_address: String,
    timestamp_block: i32,
) -> Result<(), anyhow::Error> {
    let mut transfer = sqlx_client
        .get_transfer_by_contract_address(&contract_address)
        .await?;

    transfer.timestamp_block_updated_at = timestamp_block;
    let (status, ton_pub_key) = match status {
        TonStatusEvent::ConfirmTon(x) => {
            transfer.confirm_votes += 1;
            if transfer.confirm_votes == transfer.required_votes {
                transfer.status = TransferStatus::Confirmed.to_string();
            }
            (
                TransferStatus::Confirmed.to_string(),
                x.relay.as_slice().to_vec(),
            )
        }
        TonStatusEvent::RejectTon(x) => {
            transfer.reject_votes += 1;
            if transfer.reject_votes == transfer.required_votes {
                transfer.status = TransferStatus::Rejected.to_string();
            }
            (
                TransferStatus::Rejected.to_string(),
                x.relay.as_slice().to_vec(),
            )
        }
        TonStatusEvent::ConfirmEth(x) => {
            transfer.confirm_votes += 1;
            if transfer.confirm_votes == transfer.required_votes {
                transfer.status = TransferStatus::Confirmed.to_string();
            }
            (
                TransferStatus::Confirmed.to_string(),
                x.relay.as_slice().to_vec(),
            )
        }
        TonStatusEvent::RejectEth(x) => {
            transfer.reject_votes += 1;
            if transfer.reject_votes == transfer.required_votes {
                transfer.status = TransferStatus::Rejected.to_string();
            }
            (
                TransferStatus::Rejected.to_string(),
                x.relay.as_slice().to_vec(),
            )
        }
    };

    let (relay_user_address, ton_pub_key) = sqlx_client
        .get_relay_user_address_by_key(&ton_pub_key)
        .await?;

    let relay_event = RelayEventFromDb {
        relay_user_address,
        contract_address: contract_address.clone(),
        ton_pub_key,
        transfer_user_address: transfer.user_address.clone(),
        status: status.clone(),
        volume_exec: transfer.volume_exec,
        transfer_kind: transfer.transfer_kind.clone(),
        currency_address: transfer.ton_token_address.to_string(),
        timestamp_block,
    };
    sqlx_client.new_relay_event(relay_event, transfer).await
}

pub async fn parse_receive_round_relay(
    sqlx_client: SqlxClient,
    contract_address: MsgAddressInt,
    node: &TransactionProducer,
    abi_type: AbiType,
) -> Result<(), anyhow::Error> {
    let id = build_answer_id_camel();

    let required_votes: i32 = match abi_type {
        AbiType::Ton => {
            let function_output = node
                .run_local(&contract_address, &get_ton_details_function(), &[id])
                .await?
                .context("none function output")?;
            let details: crate::models::abi::token_transfer_ton_event::GetDetails =
                function_output.tokens.unwrap_or_default().unpack()?;
            details.required_votes as i32
        }
        AbiType::Ethereum => {
            let function_output = node
                .run_local(&contract_address, &get_eth_details_function(), &[id])
                .await?
                .context("none function output")?;
            let details: crate::models::abi::token_transfer_ethereum_event::GetDetails =
                function_output.tokens.unwrap_or_default().unpack()?;
            details.required_votes as i32
        }
    };

    sqlx_client
        .update_required_votes(&contract_address.to_string(), required_votes)
        .await
}

fn get_ton_decoded_data_function() -> Function {
    let abi = ton_abi::Contract::load(std::io::Cursor::new(TOKEN_TRANSFER_TON_EVENT)).trust_me();
    abi.function("getDecodedData").trust_me().clone()
}

fn get_ton_details_function() -> Function {
    let abi = ton_abi::Contract::load(std::io::Cursor::new(TOKEN_TRANSFER_TON_EVENT)).trust_me();
    abi.function("getDetails").trust_me().clone()
}

fn get_eth_details_function() -> Function {
    let abi =
        ton_abi::Contract::load(std::io::Cursor::new(TOKEN_TRANSFER_ETHEREUM_EVENT)).trust_me();
    abi.function("getDetails").trust_me().clone()
}

fn get_eth_decoded_function() -> Function {
    let abi =
        ton_abi::Contract::load(std::io::Cursor::new(TOKEN_TRANSFER_ETHEREUM_EVENT)).trust_me();
    abi.function("getDecodedData").trust_me().clone()
}

fn get_details_eth_configuration_function() -> Function {
    let abi =
        ton_abi::Contract::load(std::io::Cursor::new(ETHEREUM_EVENT_CONFIGURATION)).trust_me();
    abi.function("getDetails").trust_me().clone()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::io::Write;
    use std::str::FromStr;

    use chrono::Local;
    use env_logger::init;
    use env_logger::Builder;
    use hex::ToHex;
    use log::LevelFilter;

    use super::*;

    #[tokio::test]
    async fn test_get_details() {
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} {}{}{} [{}] - {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.module_path().unwrap_or_default(),
                    record.file().unwrap_or_default(),
                    record.line().unwrap_or_default(),
                    record.level(),
                    record.args(),
                )
            })
            .filter(None, LevelFilter::Info)
            .init();
        let transaction_producer = TransactionProducer::new(
            "ton-bridge-local-test-2",
            "ton-transactions".into(),
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
        let id = build_answer_id_camel();
        // let function_output = transaction_producer
        //     .run_local(
        //         &MsgAddressInt::from_str(
        //             "0:dc87a98a2324885c679ef016e3c663d2083703522901413bf693e184189c2fde",
        //         )
        //         .unwrap(),
        //         &get_ton_details_function(),
        //         &[id.clone()],
        //     )
        //     .await
        //     .unwrap()
        //     .context("none function output")
        //     .unwrap();

        // log::error!(
        //     "tokens: {:#?}",
        //     function_output.tokens.clone().unwrap_or_default()
        // );
        // let details: crate::models::abi::token_transfer_ton_event::GetDetails =
        //     function_output.tokens.unwrap_or_default().unpack().unwrap();
        let function_output = transaction_producer
            .run_local(
                &MsgAddressInt::from_str(
                    "0:5b79658360bb49bdd30fff40b1ae318aaa631f114cb421f9a9af85a6385e5b02",
                )
                .unwrap(),
                &get_eth_details_function(),
                &[id.clone()],
            )
            .await
            .unwrap()
            .context("none function output")
            .unwrap();
        let details: crate::models::abi::token_transfer_ethereum_event::GetDetails =
            function_output.tokens.unwrap_or_default().unpack().unwrap();
        println!("{:#?}", details);
        // println!("initializer 0x{}", hex::encode(details.network_configuration.event_emitter));
        // let function_output = transaction_producer
        //     .run_local(
        //         &details.event_init_data.configuration,
        //         &get_details_eth_configuration_function(),
        //         &[id],
        //     )
        //     .await
        //     .unwrap()
        //     .context("none function output")
        //     .unwrap();
        // println!("{:#?}", function_output);
        // let details: crate::models::abi::ethereum_event_configuration::GetDetails =
        //     function_output.tokens.unwrap_or_default().unpack().unwrap();
        // println!("0x{}", hex::encode(details.network_configuration.event_emitter));
    }
}
