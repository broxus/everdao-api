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
