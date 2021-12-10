use nekoton_abi::*;
use once_cell::sync::OnceCell;

/// External responsible function
pub fn expected_proposal_address() -> &'static ton_abi::Function {
    static FUNCTION: OnceCell<ton_abi::Function> = OnceCell::new();
    FUNCTION.get_or_init(|| {
        FunctionBuilder::new_responsible("expectedProposalAddress")
            .time_header()
            .input("proposalId", ton_abi::ParamType::Uint(32))
            .output("address", ton_abi::ParamType::Address)
            .build()
    })
}
