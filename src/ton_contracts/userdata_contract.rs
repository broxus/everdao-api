use nekoton_abi::*;
use once_cell::sync::OnceCell;

use crate::models::*;

/// External responsible function
pub fn get_user_data_details() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("getDetails")
            .time_header()
            .outputs(GetDetails::param_type())
            .build()
    })
}
