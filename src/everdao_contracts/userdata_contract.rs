use nekoton_abi::*;

use crate::models::*;

/// External responsible function
pub fn get_user_data_details() -> &'static ton_abi::Function {
    crate::once!(ton_abi::Function, || {
        FunctionBuilder::new_responsible("getDetails")
            .default_headers()
            .output("details", GetDetails::param_type())
            .build()
    })
}
