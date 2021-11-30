use indexer_lib::AnyExtractable;

use crate::indexer::*;

pub struct AllEvents {
    pub dao_root: Vec<AnyExtractable>,
}

impl Default for AllEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            dao_root: dao_root(),
        }
    }

    pub fn get_all_events(&self) -> Vec<AnyExtractable> {
        let res = self.dao_root.clone();
        res
    }
}

fn dao_root() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(DAO_ROOT_ABI)).unwrap();
    let events = contract.events();
    let proposal_created = events.get("ProposalCreated").unwrap();

    let contract = ton_abi::Contract::load(std::io::Cursor::new(USERDATA_ABI)).unwrap();
    let events = contract.events();
    let vote_cast = events.get("VoteCast").unwrap();

    vec![AnyExtractable::Event(proposal_created.clone()), AnyExtractable::Event(vote_cast.clone())]
}
