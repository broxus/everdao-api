use std::collections::HashSet;

use indexer_lib::AnyExtractable;
use nekoton_utils::TrustMe;

use crate::indexer::*;

pub struct AllEvents {
    pub proposal: EventsParsing,
    pub dao_root: EventsParsing,
    pub user_data: EventsParsing,
}

#[derive(Clone)]
pub struct EventsParsing {
    pub any_extractable: Vec<AnyExtractable>,
    pub events_check: HashSet<(String, u32)>,
    pub functions_check: HashSet<(String, u32)>,
}

impl EventsParsing {
    pub fn new(any_extractable: Vec<AnyExtractable>) -> Self {
        let (events_check, functions_check) = any_extractable.clone().into_iter().fold(
            (HashSet::new(), HashSet::new()),
            |(mut events, mut functions), x| {
                match x {
                    AnyExtractable::Function(a) => {
                        functions.insert((a.name.clone(), a.get_function_id()))
                    }
                    AnyExtractable::Event(a) => {
                        events.insert((a.name.clone(), a.get_function_id()))
                    }
                };
                (events, functions)
            },
        );
        Self {
            any_extractable,
            events_check,
            functions_check,
        }
    }
}

impl Default for AllEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl AllEvents {
    pub fn new() -> Self {
        Self {
            proposal: EventsParsing::new(proposal()),
            dao_root: EventsParsing::new(dao_root()),
            user_data: EventsParsing::new(user_data()),
        }
    }

    pub fn get_all_events(&self) -> EventsParsing {
        let mut res = self.proposal.any_extractable.clone();
        res.extend(self.dao_root.any_extractable.clone());
        res.extend(self.user_data.any_extractable.clone());
        EventsParsing::new(res)
    }
}

fn dao_root() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(DAO_ROOT_ABI).trust_me();
    let events = contract.events;
    let proposal_created = events.get("ProposalCreated").trust_me();

    vec![AnyExtractable::Event(proposal_created.clone())]
}

fn user_data() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(USERDATA_ABI).trust_me();
    let events = contract.events;
    let vote_cast = events.get("VoteCast").trust_me();
    let unlock_casted_votes = events.get("UnlockCastedVotes").trust_me();

    vec![
        AnyExtractable::Event(vote_cast.clone()),
        AnyExtractable::Event(unlock_casted_votes.clone()),
    ]
}

fn proposal() -> Vec<AnyExtractable> {
    let contract = ton_abi::Contract::load(PROPOSAL_ABI).trust_me();
    let events = contract.events;
    let executed = events.get("Executed").trust_me();
    let canceled = events.get("Canceled").trust_me();
    let queued = events.get("Queued").trust_me();

    vec![
        AnyExtractable::Event(executed.clone()),
        AnyExtractable::Event(canceled.clone()),
        AnyExtractable::Event(queued.clone()),
    ]
}
