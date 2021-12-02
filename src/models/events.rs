use indexer_lib::AnyExtractable;

use crate::indexer::*;

pub struct AllEvents {
    dao_root: Vec<AnyExtractable>,
    user_data: Vec<AnyExtractable>,
    proposal: Vec<AnyExtractable>,
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            dao_root: dao_root(),
            user_data: user_data(),
            proposal: proposal(),
        }
    }

    pub fn get_all_events(&self) -> Vec<AnyExtractable> {
        self.dao_root
            .clone()
            .into_iter()
            .chain(self.user_data.clone().into_iter())
            .chain(self.proposal.clone().into_iter())
            .collect()
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

fn proposal() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(std::io::Cursor::new(PROPOSAL_ABI)).unwrap();
    let events = contract.events();
    let executed = events.get("Executed").unwrap();
    let canceled = events.get("Canceled").unwrap();
    let queued = events.get("Queued").unwrap();

    vec![
        AnyExtractable::Event(executed.clone()),
        AnyExtractable::Event(canceled.clone()),
        AnyExtractable::Event(queued.clone()),
    ]
}
