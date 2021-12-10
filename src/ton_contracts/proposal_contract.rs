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
pub fn get_config() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("getConfig")
            .time_header()
            .output("config", ProposalConfig::param_type())
            .build()
    })
}

/// External responsible function
pub fn get_id() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("id")
            .time_header()
            .output("id", ton_abi::ParamType::Uint(32))
            .build()
    })
}
