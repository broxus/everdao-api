use indexer_lib::{split, AnyExtractableOutput, ParsedOutput, TransactionExt};
use nekoton_abi::UnpackAbiPlain;
use nekoton_utils::TrustMe;
use ton_consumer::TransactionProducer;

use crate::models::*;
use crate::sqlx_client::*;

use super::parse_dao_events::*;

pub async fn extract_dao_root_parsed_events(
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = events.transaction;
    let (_, events) = split(events.output);
    let timestamp_block = transaction.time() as i32;

    let contract_address = transaction.contract_address().trust_me();
    for event in events {
        let message_hash = event.message_hash.to_vec();
        let transaction_hash = transaction.tx_hash().trust_me().as_slice().to_vec();
        match event.function_name.as_str() {
            "ProposalCreated" => {
                let data: ProposalCreated = event.input.unpack()?;
                parse_proposal_created_event(
                    data,
                    timestamp_block,
                    message_hash,
                    transaction_hash,
                    contract_address.clone(),
                    sqlx_client,
                    node,
                )
                .await?;
            },
            "VoteCast" => {
                let data: VoteCast = event.input.unpack()?;
                parse_vote_cast_event(
                    data,
                    timestamp_block,
                    message_hash,
                    transaction_hash,
                    sqlx_client,
                    node,
                )
                .await?;
            }
            _ => {}
        }
    }
    Ok(())
}
