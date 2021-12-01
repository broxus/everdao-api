use indexer_lib::AnyExtractable;

use crate::indexer::*;

pub struct AllEvents {
    dao_root: Vec<AnyExtractable>,
    user_data: Vec<AnyExtractable>,
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            dao_root: dao_root(),
            user_data: user_data(),
        }
    }

    pub fn get_all_events(&self) -> &[AnyExtractable] {
        [&self.dao_root, &self.user_data].concat()
    }
}

fn dao_root() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(DAO_ROOT_ABI)).unwrap();
    let events = contract.events();
    let proposal_created = events.get("ProposalCreated").unwrap();

    vec![AnyExtractable::Event(proposal_created.clone())]
}

fn user_data() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(USERDATA_ABI)).unwrap();
    let events = contract.events();
    let vote_cast = events.get("VoteCast").unwrap();

    vec![AnyExtractable::Event(vote_cast.clone())]
}
