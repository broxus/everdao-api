use crate::indexer::abi::{
    ETHEREUM_EVENT_CONFIGURATION, STAKING_ABI, TOKEN_TRANSFER_ETHEREUM_EVENT,
    TOKEN_TRANSFER_TON_EVENT, TON_EVENT_CONFIGURATION, USER_DATA_ABI,
};
use indexer_lib::AnyExtractable;

pub struct AllEvents {
    pub staking: Vec<AnyExtractable>,
    pub user_data: Vec<AnyExtractable>,
    pub ethereum_event_configuration: Vec<AnyExtractable>,
    pub token_transfer_ethereum_event: Vec<AnyExtractable>,
    pub token_transfer_ton_even: Vec<AnyExtractable>,
    pub ton_event_configuration: Vec<AnyExtractable>,
}

impl Default for AllEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            staking: staking(),
            user_data: user_data(),
            ethereum_event_configuration: ethereum_event_configuration(),
            token_transfer_ethereum_event: token_transfer_ethereum_event(),
            token_transfer_ton_even: token_transfer_ton_even(),
            ton_event_configuration: ton_event_configuration(),
        }
    }

    pub fn get_all_events(&self) -> Vec<AnyExtractable> {
        let mut res = self.staking.clone();
        res.extend(self.user_data.clone());
        res.extend(self.ethereum_event_configuration.clone());
        res.extend(self.token_transfer_ethereum_event.clone());
        res.extend(self.token_transfer_ton_even.clone());
        res.extend(self.ton_event_configuration.clone());
        res
    }
}

fn staking() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(STAKING_ABI)).unwrap();
    let events = contract.events();
    let deposit_event = events.get("Deposit").unwrap();
    let withdraw_event = events.get("Withdraw").unwrap();
    let reward_claimed_event = events.get("RewardClaimed").unwrap();
    let new_reward_round_event = events.get("NewRewardRound").unwrap();

    vec![
        AnyExtractable::Event(deposit_event.clone()),
        AnyExtractable::Event(withdraw_event.clone()),
        AnyExtractable::Event(reward_claimed_event.clone()),
        AnyExtractable::Event(new_reward_round_event.clone()),
    ]
}

fn user_data() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(USER_DATA_ABI)).unwrap();
    let events = contract.events();
    let data_relay_keys_updated_event = events.get("RelayKeysUpdated").unwrap();
    let data_ton_pubkey_confirmed_event = events.get("TonPubkeyConfirmed").unwrap();
    let data_eth_address_confirmed_event = events.get("EthAddressConfirmed").unwrap();
    let data_relay_membership_requested_event = events.get("RelayMembershipRequested").unwrap();

    vec![
        AnyExtractable::Event(data_relay_keys_updated_event.clone()),
        AnyExtractable::Event(data_ton_pubkey_confirmed_event.clone()),
        AnyExtractable::Event(data_eth_address_confirmed_event.clone()),
        AnyExtractable::Event(data_relay_membership_requested_event.clone()),
    ]
}

fn ethereum_event_configuration() -> Vec<AnyExtractable> {
    let contract =
        ton_abi::Contract::load(std::io::Cursor::new(ETHEREUM_EVENT_CONFIGURATION)).unwrap();
    let events = contract.events();
    let new_event_contract = events.get("NewEventContract").unwrap();

    vec![AnyExtractable::Event(new_event_contract.clone())]
}

fn token_transfer_ethereum_event() -> Vec<AnyExtractable> {
    let contract =
        ton_abi::Contract::load(std::io::Cursor::new(TOKEN_TRANSFER_ETHEREUM_EVENT)).unwrap();
    let events = contract.events();
    let functions = contract.functions();
    let confirm_event = events.get("Confirm").unwrap();
    let reject_event = events.get("Reject").unwrap();
    let receive_round_relays = functions.get("receiveRoundRelays").unwrap();

    vec![
        AnyExtractable::Event(confirm_event.clone()),
        AnyExtractable::Event(reject_event.clone()),
        AnyExtractable::Function(receive_round_relays.clone()),
    ]
}

fn token_transfer_ton_even() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(TOKEN_TRANSFER_TON_EVENT)).unwrap();
    let events = contract.events();
    let functions = contract.functions();
    let confirm_event = events.get("Confirm").unwrap();
    let reject_event = events.get("Reject").unwrap();
    let receive_round_relays = functions.get("receiveRoundRelays").unwrap();

    vec![
        AnyExtractable::Event(confirm_event.clone()),
        AnyExtractable::Event(reject_event.clone()),
        AnyExtractable::Function(receive_round_relays.clone()),
    ]
}

fn ton_event_configuration() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(TON_EVENT_CONFIGURATION)).unwrap();
    let events = contract.events();
    let new_event_contract = events.get("NewEventContract").unwrap();

    vec![AnyExtractable::Event(new_event_contract.clone())]
}
