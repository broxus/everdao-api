use std::collections::hash_map;

use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tiny_adnl::utils::FxHashMap;

use crate::models::*;

static PROPOSAL_CACHE: Lazy<RwLock<FxHashMap<i32, Vec<ProposalActionType>>>> =
    Lazy::new(|| RwLock::new(FxHashMap::with_hasher(Default::default())));

#[derive(Clone)]
pub enum ProposalActionType {
    Executed(i32),
    Canceled(i32),
    Queued(i32, i64),
    Vote(UpdateProposalVotes),
}

pub fn save_proposal_action_in_cache(proposal_id: i32, action: ProposalActionType) {
    match PROPOSAL_CACHE.write().entry(proposal_id) {
        hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![action]);
        }
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().push(action);
        }
    }
}

pub fn remove_proposal_actions_from_cache(proposal_id: i32) -> Result<Vec<ProposalActionType>> {
    PROPOSAL_CACHE
        .write()
        .remove(&proposal_id)
        .ok_or_else(|| GlobalProposalCacheError::ProposalNotFound(proposal_id).into())
}

#[derive(thiserror::Error, Debug)]
enum GlobalProposalCacheError {
    #[error("Proposal `{0}` not found in the cache")]
    ProposalNotFound(i32),
}
