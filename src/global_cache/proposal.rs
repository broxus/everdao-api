use std::collections::{hash_map, HashMap};

use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::models::*;

static PROPOSAL_CACHE: Lazy<RwLock<HashMap<i32, Vec<ProposalActionType>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone)]
pub enum ProposalActionType {
    Executed(i32),
    Canceled(i32),
    Queued(i32, i64),
    Vote(UpdateProposalVotes),
}

pub fn save_proposal_action_in_cache(propsal_id: i32, action: ProposalActionType) {
    match PROPOSAL_CACHE.write().entry(propsal_id) {
        hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![action]);
        }
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().push(action);
        }
    }
}

pub fn get_proposal_actions_from_cache(propsal_id: i32) -> Result<Vec<ProposalActionType>> {
    let res = PROPOSAL_CACHE
        .read()
        .get(&propsal_id)
        .ok_or(GlobalCacheError::ProposalNotFound(propsal_id))?
        .to_vec();

    PROPOSAL_CACHE.write().remove(&propsal_id);

    Ok(res)
}

#[derive(thiserror::Error, Debug)]
enum GlobalCacheError {
    #[error("Proposal `{0}` not found in global cache")]
    ProposalNotFound(i32),
}
