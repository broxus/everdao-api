use std::collections::hash_map;

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tiny_adnl::utils::FxHashMap;
use ton_block::MsgAddressInt;

static PROPOSAL_CACHE: Lazy<RwLock<FxHashMap<MsgAddressInt, Vec<ProposalActionType>>>> =
    Lazy::new(|| RwLock::new(FxHashMap::with_hasher(Default::default())));

#[derive(Clone)]
pub enum ProposalActionType {
    Executed(i32),
    Canceled(i32),
    Queued(i32, i64),
}

pub fn save_proposal_action_in_cache(proposal_address: MsgAddressInt, action: ProposalActionType) {
    match PROPOSAL_CACHE.write().entry(proposal_address) {
        hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![action]);
        }
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().push(action);
        }
    }
}

pub fn remove_proposal_actions_from_cache(
    proposal_address: &MsgAddressInt,
) -> Vec<ProposalActionType> {
    PROPOSAL_CACHE
        .write()
        .remove(proposal_address)
        .unwrap_or_default()
}

pub fn is_proposal_cache_empty() -> bool {
    PROPOSAL_CACHE.read().is_empty()
}
