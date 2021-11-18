pub enum TonStatusEvent {
    ConfirmTon(crate::models::abi::token_transfer_ton_event::Confirm),
    RejectTon(crate::models::abi::token_transfer_ton_event::Reject),
    ConfirmEth(crate::models::abi::token_transfer_ethereum_event::Confirm),
    RejectEth(crate::models::abi::token_transfer_ethereum_event::Reject),
}

impl ToString for TonStatusEvent {
    fn to_string(&self) -> String {
        match self {
            TonStatusEvent::ConfirmTon(_) => "ConfirmTon".to_string(),
            TonStatusEvent::RejectTon(_) => "RejectTon".to_string(),
            TonStatusEvent::ConfirmEth(_) => "ConfirmEth".to_string(),
            TonStatusEvent::RejectEth(_) => "RejectEth".to_string(),
        }
    }
}
