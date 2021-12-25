use nekoton_abi::*;

use crate::models::*;

/// External responsible function
pub fn get_overview() -> &'static ton_abi::Function {
    crate::once!(ton_abi::Function, || {
        FunctionBuilder::new_responsible("getOverview")
            .time_header()
            .outputs(ProposalOverview::param_type())
            .build()
    })
}

/// External responsible function
pub fn get_config() -> &'static ton_abi::Function {
    crate::once!(ton_abi::Function, || {
        FunctionBuilder::new_responsible("getConfig")
            .time_header()
            .output("config", ProposalConfig::param_type())
            .build()
    })
}

pub fn get_id() -> &'static ton_abi::Function {
    crate::once!(ton_abi::Function, || {
        FunctionBuilder::new("id")
            .time_header()
            .output("id", ton_abi::ParamType::Uint(32))
            .build()
    })
}

pub fn get_dao_root() -> &'static ton_abi::Function {
    crate::once!(ton_abi::Function, || {
        FunctionBuilder::new("root")
            .time_header()
            .output("address", ton_abi::ParamType::Address)
            .build()
    })
}
