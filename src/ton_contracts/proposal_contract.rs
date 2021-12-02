use nekoton_abi::*;
use once_cell::sync::OnceCell;

use crate::models::*;

/// External responsible function
pub fn get_overview() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("getOverview")
            .time_header()
            .outputs(ProposalOverview::param_type())
            .build()
    })
}

/// External responsible function
pub fn get_id() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("id")
            .time_header()
            .outputs(ProposalId::param_type())
            .build()
    })
}

/// External responsible function
pub fn get_config() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("getConfig")
            .time_header()
            .outputs(GetProposalConfig::param_type())
            .build()
    })
}

/// External responsible function
pub fn get_actions() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("getActions")
            .time_header()
            .outputs(GetActions::param_type())
            .build()
    })
}

pub mod events {
    use super::*;

    pub fn proposal_executed() -> &'static ton_abi::Event {
        static EVENT: OnceCell<ton_abi::Event> = OnceCell::new();
        EVENT.get_or_init(|| {
            EventBuilder::new("Executed")
                .inputs(ProposalExecuted::param_type())
                .build()
        })
    }
    pub fn proposal_canceled() -> &'static ton_abi::Event {
        static EVENT: OnceCell<ton_abi::Event> = OnceCell::new();
        EVENT.get_or_init(|| {
            EventBuilder::new("Canceled")
                .inputs(ProposalCanceled::param_type())
                .build()
        })
    }
    pub fn proposal_queued() -> &'static ton_abi::Event {
        static EVENT: OnceCell<ton_abi::Event> = OnceCell::new();
        EVENT.get_or_init(|| {
            EventBuilder::new("Queued")
                .inputs(ProposalQueued::param_type())
                .build()
        })
    }
}
