use anyhow::Context;
use chrono::Utc;
use indexer_lib::TransactionExt;
use nekoton_abi::num_traits::Zero;
use nekoton_abi::UnpackAbiPlain;
use nekoton_utils::TrustMe;
use rust_decimal::Decimal;
use ton_abi::Function;
use ton_block::{MsgAddressInt, Transaction};
use ton_consumer::TransactionProducer;

use crate::indexer::abi::{STAKING_ABI, USER_DATA_ABI};
use crate::indexer::utils::{build_answer_id_camel, BRIDGE_SCALE, STAKING_CONTRACT_ADDRESS};
use crate::models::abi::staking::{NewRewardRound, RewardRound};
use crate::models::abi::user_data::{RelayKeysUpdated, RelayMembershipRequested};
use crate::models::address_type::UserAddressType;
use crate::models::event_type::{BalanceEvent, EventType};
use crate::models::sqlx::{
    BridgeBalanceFromDb, ProposalFromDb, RewardRoundInfoFromDb, UnknownUserKeysFromDb,
    UserKeysFromDb, VoteFromDb,
};
use crate::sqlx_client::SqlxClient;

pub async fn parse_freeze_stake_event(
    sqlx_client: SqlxClient,
    frozen_event: RelayMembershipRequested,
    timestamp_block: i32,
    message_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let user_address = sqlx_client
        .get_user_address_by_keys(
            frozen_event.ton_pubkey.as_slice(),
            &frozen_event.eth_address,
        )
        .await?;

    let last_user_info = sqlx_client
        .get_last_user_info(&user_address)
        .await
        .map_err(|e| {
            log::error!("{} zero info {}", user_address, e);
            e
        })
        .unwrap_or_default();

    sqlx_client
        .new_frozen_transaction(
            &user_address,
            frozen_event.lock_until as i32,
            last_user_info.stake_balance,
            VoteFromDb {
                message_hash,
                transaction_hash,
                transaction_kind: EventType::Freeze.to_string(),
                user_address: user_address.clone(),
                user_public_key: None,
                bridge_exec: Default::default(),
                timestamp_block,
                created_at: 0,
            },
        )
        .await
}

pub async fn parse_update_user_keys(
    sqlx_client: SqlxClient,
    update_key_event: RelayKeysUpdated,
    node: &TransactionProducer,
    transaction: Transaction,
) -> Result<(), anyhow::Error> {
    let contract_address = transaction.contract_address().unwrap().to_string();
    let user_address = get_user_address_from_update_event(&contract_address, node)
        .await
        .unwrap_or_default()
        .to_string();
    let mut update_user_keys = UserKeysFromDb {
        user_address,
        ton_pubkey: update_key_event.ton_pubkey.as_slice().to_vec(),
        ton_pubkey_is_confirmed: false,
        eth_address: update_key_event.eth_address.to_vec(),
        eth_address_is_confirmed: false,
        until_frozen: 0,
        updated_at: 0,
        created_at: 0,
    };
    update_user_keys.eth_address_is_confirmed = sqlx_client
        .is_key_already_confirmed(update_user_keys.eth_address.clone())
        .await;
    update_user_keys.ton_pubkey_is_confirmed = sqlx_client
        .is_key_already_confirmed(update_user_keys.ton_pubkey.clone())
        .await;

    sqlx_client.upsert_user_keys(update_user_keys).await
}

pub async fn parse_confirm_key_event(
    sqlx_client: SqlxClient,
    key_address: Vec<u8>,
    kind: UserAddressType,
) -> Result<(), anyhow::Error> {
    sqlx_client
        .new_unknown_key(UnknownUserKeysFromDb {
            address: key_address,
            kind: kind.to_string(),
        })
        .await
}

pub async fn parse_balance_event(
    sqlx_client: SqlxClient,
    balance_event: BalanceEvent,
    timestamp_block: i32,
    message_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let (user_address, mut balance_diff, mut reward_diff, event_type) = match balance_event {
        BalanceEvent::Deposit(x) => (
            x.user.to_string(),
            Decimal::from(x.amount),
            Decimal::zero(),
            EventType::Deposit,
        ),
        BalanceEvent::Withdraw(x) => (
            x.user.to_string(),
            -Decimal::from(x.amount),
            Decimal::zero(),
            EventType::Withdraw,
        ),
        BalanceEvent::Claim(x) => (
            x.user.to_string(),
            Decimal::zero(),
            Decimal::from(x.reward_tokens),
            EventType::Claim,
        ),
    };

    balance_diff.set_scale(BRIDGE_SCALE).unwrap();
    reward_diff.set_scale(BRIDGE_SCALE).unwrap();

    let transaction = VoteFromDb {
        message_hash: message_hash.clone(),
        transaction_hash: transaction_hash.clone(),
        transaction_kind: event_type.to_string(),
        user_address: user_address.clone(),
        user_public_key: None,
        bridge_exec: balance_diff + reward_diff,
        timestamp_block,
        created_at: 0,
    };

    let last_user_info = sqlx_client
        .get_last_user_info(&user_address)
        .await
        .unwrap_or_default();
    let new_user_balance = last_user_info.stake_balance + balance_diff;

    let last_bridge_balance = sqlx_client.get_last_bride_balance().await?;

    let stakeholders = if new_user_balance.is_zero() && event_type != EventType::Claim {
        last_bridge_balance.stakeholders - 1
    } else if last_user_info.stake_balance.is_zero() {
        last_bridge_balance.stakeholders + 1
    } else {
        last_bridge_balance.stakeholders
    };

    let new_balance_row = BridgeBalanceFromDb {
        message_hash,
        transaction_hash,
        transaction_kind: event_type.to_string(),
        user_address: user_address.clone(),
        user_balance: new_user_balance,
        reward: match reward_diff.is_zero() {
            true => None,
            false => Some(reward_diff),
        },
        bridge_balance: last_bridge_balance.bridge_balance + balance_diff,
        stakeholders,
        average_apr: last_bridge_balance.average_apr, //todo!
        bridge_reward: last_bridge_balance.bridge_reward + reward_diff,
        timestamp_block,
        created_at: 0,
    };

    let user_balance = ProposalFromDb {
        user_address,
        user_kind: last_user_info.user_kind,
        stake_balance: new_user_balance,
        frozen_stake: match last_user_info.until_frozen > Utc::now().timestamp() as i32 {
            true => new_user_balance,
            false => Decimal::zero(),
        },
        last_reward: reward_diff,
        total_reward: last_user_info.total_reward + reward_diff,
        until_frozen: last_user_info.until_frozen,
        updated_at: 0,
        created_at: 0,
    };

    sqlx_client
        .new_balance_transaction(transaction, new_balance_row, user_balance)
        .await
}

pub async fn parse_new_round_event(
    sqlx_client: SqlxClient,
    new_round: NewRewardRound,
    node: &TransactionProducer,
) -> Result<(), anyhow::Error> {
    let id = build_answer_id_camel();
    let function_output = node
        .run_local(
            &STAKING_CONTRACT_ADDRESS.parse()?,
            &get_staking_get_details_function(),
            &[id],
        )
        .await?
        .context("none function output")?;
    let details: crate::models::abi::staking::GetDetails =
        function_output.tokens.unwrap_or_default().unpack()?;
    if sqlx_client
        .get_all_reward_rounds()
        .await
        .unwrap_or_default()
        .is_empty()
    {
        let mut num_round = 0;
        let rounds_from_db = details
            .reward_rounds
            .into_iter()
            .fold(vec![], |mut res, x| {
                let mut reward_tokens = Decimal::from(x.reward_tokens);
                reward_tokens.set_scale(BRIDGE_SCALE).unwrap();
                let mut total_reward = Decimal::from(x.total_reward);
                total_reward.set_scale(BRIDGE_SCALE).unwrap();

                let round = RewardRoundInfoFromDb {
                    num_round,
                    start_time: x.start_time as i32,
                    end_time: i32::MAX,
                    reward_tokens,
                    total_reward,
                };
                num_round += 1;
                res.push(round);
                res
            });
        for round in rounds_from_db {
            if let Err(e) = sqlx_client.new_reward_round(round.clone()).await {
                log::error!("{} {:#?}", e, round);
            }
        }
    } else {
        let round_event: RewardRound = details
            .reward_rounds
            .get(new_round.round_num as usize)
            .unwrap()
            .clone();
        let mut reward_tokens = Decimal::from(round_event.reward_tokens);
        reward_tokens.set_scale(BRIDGE_SCALE).unwrap();
        let mut total_reward = Decimal::from(round_event.total_reward);
        total_reward.set_scale(BRIDGE_SCALE).unwrap();

        let round_from_db = RewardRoundInfoFromDb {
            num_round: new_round.round_num as i32,
            start_time: round_event.start_time as i32,
            end_time: i32::MAX,
            reward_tokens,
            total_reward,
        };
        if let Err(e) = sqlx_client.new_reward_round(round_from_db.clone()).await {
            log::error!("{} {:#?}", e, round_from_db);
        }
    }

    Ok(())
}

fn get_staking_get_details_function() -> Function {
    let abi = ton_abi::Contract::load(std::io::Cursor::new(STAKING_ABI)).trust_me();
    abi.function("getDetails").trust_me().clone()
}

async fn get_user_address_from_update_event(
    contract_address: &str,
    node: &TransactionProducer,
) -> Result<MsgAddressInt, anyhow::Error> {
    let id = build_answer_id_camel();

    let function_output = node
        .run_local(
            &contract_address.parse()?,
            &get_user_data_get_details_function(),
            &[id],
        )
        .await?
        .context("none function output")?;
    let details: crate::models::abi::user_data::GetDetails =
        function_output.tokens.unwrap_or_default().unpack()?;
    Ok(details.value0.user)
}

fn get_user_data_get_details_function() -> Function {
    let abi = ton_abi::Contract::load(std::io::Cursor::new(USER_DATA_ABI)).trust_me();
    abi.function("getDetails").trust_me().clone()
}

// async fn get_owner(
//     node: &NodeClient,
//     address: MsgAddressInt,
// ) -> Result<MsgAddressInt, anyhow::Error> {
//     let state = node.get_contract_state(address.clone()).await?;
//     let state = match state {
//         RawContractState::NotExists => {
//             anyhow::bail!("{} contracts doesn't exist", &address)
//         }
//         RawContractState::Exists(a) => a,
//     };
//
//     let state = RootTokenContractState(&state);
//     Ok(state
//         .guess_details()
//         .context("Failed guessing details")?
//         .owner_address)
// }
