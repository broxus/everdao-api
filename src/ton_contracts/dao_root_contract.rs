use nekoton_abi::*;
use once_cell::sync::OnceCell;

use crate::models::*;

pub mod events {
    use super::*;

    pub fn proposal_created() -> &'static ton_abi::Event {
        static EVENT: OnceCell<ton_abi::Event> = OnceCell::new();
        EVENT.get_or_init(|| {
            EventBuilder::new("ProposalCreated")
                .inputs(ProposalCreated::param_type())
                .build()
        })
    }
}
