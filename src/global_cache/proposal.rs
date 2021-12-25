use std::collections::hash_map;

use anyhow::Result;
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
) -> Result<Vec<ProposalActionType>> {
    PROPOSAL_CACHE
        .write()
        .remove(proposal_address)
        .ok_or_else(|| {
            GlobalProposalCacheError::ProposalNotFound(proposal_address.to_string()).into()
        })
}

#[derive(thiserror::Error, Debug)]
enum GlobalProposalCacheError {
    #[error("Proposal address `{0}` not found in the cache")]
    ProposalNotFound(String),
}
