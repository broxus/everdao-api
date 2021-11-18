use crate::indexer::parse_staking_events::{
    parse_balance_event, parse_confirm_key_event, parse_freeze_stake_event, parse_new_round_event,
    parse_update_user_keys,
};
use crate::indexer::parse_transfer_events::{
    parse_ethereum_event_configuration_event, parse_receive_round_relay, parse_status_event,
    parse_ton_event_configuration_event,
};
use crate::models::abi::staking::{Deposit, NewRewardRound, RewardClaimed, Withdraw};
use crate::models::abi::user_data::{
    EthAddressConfirmed, RelayKeysUpdated, RelayMembershipRequested, TonPubkeyConfirmed,
};
use crate::models::address_type::UserAddressType;
use crate::models::event_type::BalanceEvent;
use crate::models::transfer_enums::abi_type::AbiType;
use crate::models::transfer_enums::ton_status::TonStatusEvent;
use crate::sqlx_client::SqlxClient;
use indexer_lib::{split, AnyExtractableOutput, ParsedOutput, TransactionExt};
use nekoton_abi::UnpackAbiPlain;
use nekoton_utils::TrustMe;
use ton_consumer::TransactionProducer;

pub async fn extract_staking_parsed_events(
    sqlx_client: SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (_, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    for event in events.clone() {
        let message_hash = event.message_hash.to_vec();
        match event.function_name.as_str() {
            "Deposit" => {
                let deposit: Deposit = event.clone().input.unpack()?;
                parse_balance_event(
                    sqlx_client.clone(),
                    BalanceEvent::Deposit(deposit),
                    timestamp_block,
                    message_hash,
                    transaction_hash.clone(),
                )
                .await?;
            }
            "Withdraw" => {
                let withdraw: Withdraw = event.clone().input.unpack()?;
                parse_balance_event(
                    sqlx_client.clone(),
                    BalanceEvent::Withdraw(withdraw),
                    timestamp_block,
                    message_hash,
                    transaction_hash.clone(),
                )
                .await?;
            }
            "RewardClaimed" => {
                let claim: RewardClaimed = event.clone().input.unpack()?;
                parse_balance_event(
                    sqlx_client.clone(),
                    BalanceEvent::Claim(claim),
                    timestamp_block,
                    message_hash,
                    transaction_hash.clone(),
                )
                .await?;
            }
            "NewRewardRound" => {
                let new_reward_round: NewRewardRound = event.clone().input.unpack()?;
                if let Err(e) =
                    parse_new_round_event(sqlx_client.clone(), new_reward_round, node).await
                {
                    log::error!("{}", e);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn extract_user_data_parsed_events(
    sqlx_client: SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (_, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    for event in events.clone() {
        let message_hash = event.message_hash.to_vec();
        match event.function_name.as_str() {
            "RelayKeysUpdated" => {
                let relay_keys_updated: RelayKeysUpdated = event.clone().input.unpack()?;
                if let Err(e) = parse_update_user_keys(
                    sqlx_client.clone(),
                    relay_keys_updated.clone(),
                    node,
                    transaction.clone(),
                )
                .await
                {
                    log::error!("{} {:#?}", e, relay_keys_updated);
                }
            }
            "TonPubkeyConfirmed" => {
                let ton_pub_key_confirmed: TonPubkeyConfirmed = event.clone().input.unpack()?;
                if let Err(e) = parse_confirm_key_event(
                    sqlx_client.clone(),
                    ton_pub_key_confirmed.ton_pubkey.as_slice().to_vec(),
                    UserAddressType::TonPubkey,
                )
                .await
                {
                    log::error!("{} {:#?}", e, ton_pub_key_confirmed);
                }
            }
            "EthAddressConfirmed" => {
                let eth_address_confirmed: EthAddressConfirmed = event.clone().input.unpack()?;
                if let Err(e) = parse_confirm_key_event(
                    sqlx_client.clone(),
                    eth_address_confirmed.eth_addr.to_vec(),
                    UserAddressType::EthAddress,
                )
                .await
                {
                    log::error!("{} {:#?}", e, eth_address_confirmed);
                }
            }
            "RelayMembershipRequested" => {
                let relay_membership_requested: RelayMembershipRequested =
                    event.clone().input.unpack()?;
                if let Err(e) = parse_freeze_stake_event(
                    sqlx_client.clone(),
                    relay_membership_requested.clone(),
                    timestamp_block,
                    message_hash,
                    transaction_hash.clone(),
                )
                .await
                {
                    log::error!("{} {:#?}", e, relay_membership_requested);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn extract_ethereum_event_configuration_parsed_events(
    sqlx_client: SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (_, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    for event in events.clone() {
        let message_hash = event.message_hash.to_vec();
        if event.function_name.as_str() == "NewEventContract" {
            let new_event_contract: crate::models::abi::ethereum_event_configuration::NewEventContract = event.input.unpack()?;
            if let Err(e) = parse_ethereum_event_configuration_event(
                node,
                new_event_contract.clone(),
                sqlx_client.clone(),
                timestamp_block,
                transaction_hash.clone(),
                message_hash,
            )
            .await
            {
                log::error!("{} {:#?}", e, new_event_contract)
            }
        }
    }
    Ok(())
}

pub async fn extract_token_transfer_ton_even_parsed_events(
    sqlx_client: SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (functions, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;

    for function in functions {
        if function.function_name.as_str() == "receiveRoundRelays" {
            if let Err(e) = parse_receive_round_relay(
                sqlx_client.clone(),
                transaction.contract_address().unwrap(),
                node,
                AbiType::Ton,
            )
            .await
            {
                log::error!(
                    "receiveRoundRelays error {} address {}",
                    e,
                    transaction.contract_address().unwrap().to_string()
                );
            }
        }
    }

    for event in events.clone() {
        match event.function_name.as_str() {
            "Confirm" => {
                let confirm: crate::models::abi::token_transfer_ton_event::Confirm =
                    event.clone().input.unpack()?;
                if let Err(e) = parse_status_event(
                    sqlx_client.clone(),
                    TonStatusEvent::ConfirmTon(confirm.clone()),
                    transaction
                        .contract_address()
                        .unwrap_or_default()
                        .to_string(),
                    timestamp_block,
                )
                .await
                {
                    log::error!("{} {:#?}", e, confirm);
                }
            }
            "Reject" => {
                let reject: crate::models::abi::token_transfer_ton_event::Reject =
                    event.clone().input.unpack()?;
                if let Err(e) = parse_status_event(
                    sqlx_client.clone(),
                    TonStatusEvent::RejectTon(reject.clone()),
                    transaction
                        .contract_address()
                        .unwrap_or_default()
                        .to_string(),
                    timestamp_block,
                )
                .await
                {
                    log::error!("{} {:#?}", e, reject);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn extract_token_transfer_ethereum_event_parsed_events(
    sqlx_client: SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (functions, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;

    for function in functions {
        if function.function_name.as_str() == "receiveRoundRelays" {
            if let Err(e) = parse_receive_round_relay(
                sqlx_client.clone(),
                transaction.contract_address().unwrap(),
                node,
                AbiType::Ethereum,
            )
            .await
            {
                log::error!(
                    "receiveRoundRelays error {} address {}",
                    e,
                    transaction.contract_address().unwrap().to_string()
                );
            }
        }
    }

    for event in events.clone() {
        match event.function_name.as_str() {
            "Confirm" => {
                let confirm: crate::models::abi::token_transfer_ethereum_event::Confirm =
                    event.clone().input.unpack()?;
                if let Err(e) = parse_status_event(
                    sqlx_client.clone(),
                    TonStatusEvent::ConfirmEth(confirm.clone()),
                    transaction
                        .contract_address()
                        .unwrap_or_default()
                        .to_string(),
                    timestamp_block,
                )
                .await
                {
                    log::error!("{} {:#?}", e, confirm);
                }
            }
            "Reject" => {
                let reject: crate::models::abi::token_transfer_ethereum_event::Reject =
                    event.clone().input.unpack()?;
                if let Err(e) = parse_status_event(
                    sqlx_client.clone(),
                    TonStatusEvent::RejectEth(reject.clone()),
                    transaction
                        .contract_address()
                        .unwrap_or_default()
                        .to_string(),
                    timestamp_block,
                )
                .await
                {
                    log::error!("{} {:#?}", e, reject);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn extract_ton_event_configuration_parsed_events(
    sqlx_client: SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (_, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;
    let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();

    for event in events.clone() {
        let message_hash = event.message_hash.to_vec();
        if event.function_name.as_str() == "NewEventContract" {
            let new_event_contract: crate::models::abi::ton_event_configuration::NewEventContract =
                event.input.unpack()?;
            if let Err(e) = parse_ton_event_configuration_event(
                node,
                new_event_contract.clone(),
                sqlx_client.clone(),
                timestamp_block,
                transaction_hash.clone(),
                message_hash,
            )
            .await
            {
                log::error!("{} {:#?}", e, new_event_contract)
            }
        }
    }
    Ok(())
}
