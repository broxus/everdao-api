use nekoton_abi::*;

/// External responsible function
pub fn expected_proposal_address() -> &'static ton_abi::Function {
    crate::once!(ton_abi::Function, || {
        FunctionBuilder::new_responsible("expectedProposalAddress")
            .time_header()
            .input("proposalId", ton_abi::ParamType::Uint(32))
            .output("address", ton_abi::ParamType::Address)
            .build()
    })
}
