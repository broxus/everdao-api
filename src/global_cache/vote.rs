use std::collections::HashSet;

use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::models::*;

static VOTE_CACHE: Lazy<RwLock<HashSet<UnlockVote>>> = Lazy::new(|| RwLock::new(HashSet::new()));

pub fn save_locked_vote_in_cache(vote: UnlockVote) -> Result<()> {
    if !VOTE_CACHE.write().insert(vote.clone()) {
        return Err(GlobalVoteCacheError::VoteExist(vote.proposal_id, vote.voter).into());
    }

    Ok(())
}

pub fn remove_vote_actions_from_cache(vote: UnlockVote) -> bool {
    VOTE_CACHE.write().remove(&vote)
}

#[derive(thiserror::Error, Debug)]
enum GlobalVoteCacheError {
    #[error("Vote `{0} {1}` is exist in the cache")]
    VoteExist(i32, String),
}
