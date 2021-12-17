use indexer_lib::{split, AnyExtractableOutput, ParsedOutput, TransactionExt};
use nekoton_abi::{UnpackAbiPlain, UnpackFirst};
use ton_consumer::TransactionProducer;

use super::parse_dao_events::*;
use super::parse_proposal_events::*;
use super::parse_userdata_events::*;
use crate::models::*;
use crate::sqlx_client::*;

pub async fn extract_dao_root_parsed_events(
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = &events.transaction;

    let (_, events) = split(events.output);
    for event in events {
        let message_hash = event.message_hash.to_vec();
        if event.function_name.as_str() == "ProposalCreated" {
            let data: ProposalCreated = event.input.unpack()?;
            parse_proposal_created_event(
                data,
                message_hash,
                transaction,
                sqlx_client,
                transaction_producer,
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn extract_proposal_parsed_events(
    sqlx_client: &SqlxClient,
    node: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = &events.transaction;

    let (_, events) = split(events.output);
    for event in events {
        match event.function_name.as_str() {
            "Executed" => {
                parse_proposal_executed_event(transaction, sqlx_client, node).await?;
            }
            "Canceled" => {
                parse_proposal_canceled_event(transaction, sqlx_client, node).await?;
            }
            "Queued" => {
                let execution_time: u32 = event.input.unpack_first()?;
                parse_proposal_queued_event(execution_time, transaction, sqlx_client, node).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn extract_userdata_parsed_events(
    sqlx_client: &SqlxClient,
    transaction_producer: &TransactionProducer,
    events: ParsedOutput<AnyExtractableOutput>,
) -> Result<(), anyhow::Error> {
    let transaction = &events.transaction;

    let (_, events) = split(events.output);
    for event in events {
        match event.function_name.as_str() {
            "VoteCast" => {
                let vote: VoteCast = event.input.unpack()?;
                let message_hash = event.message_hash.to_vec();
                parse_vote_cast_event(
                    vote,
                    message_hash,
                    transaction,
                    sqlx_client,
                    transaction_producer,
                )
                .await?;
            }
            "UnlockCastedVotes" => {
                let proposal_id: u32 = event.input.unpack_first()?;
                let voter = transaction.contract_address()?;
                parse_unlock_casted_votes_event(proposal_id, voter, sqlx_client).await?
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use indexer_lib::TransactionExt;
    use nekoton_utils::TrustMe;
    use ton_block::Deserializable;

    use super::*;
    use crate::indexer::bridge_dao::extract_events;

    #[test]
    fn parse_proposal_created_event() {
        let all_events = AllEvents::new();
        let transaction = ton_block::Transaction::construct_from_base64("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dG9uLWV0aC1icmlkZ2UtY29udHJhY3RzIiwiRm9sbG93aW5nIHRoZSBlYXJsaWVyIGFkb3B0jQH+ZWQgW3Byb3Bvc2FsIG9mIHRoZSBEZUZpIEFsbGlhbmNlXShodHRwczovL2dvdi5mcmVldG9uLm9yZy9wcm9wb3NhbD9pc0dsb2JhbD0wJnByb3Bvc2FsQWRkcmVzcz0wJTNBMmZhNDhlZDVhOTNiZmMwM2I4NjY3YTJlNjA1M44B/jhhYWVmMTk0YWM2OGMzMzM4ZThhMDNmMjFkOGEyZjM3ODE1OCksIEdsb2JhbCBHb3Zlcm5hbmNlIHJlbmFtZWQgdGhlIEZyZWVUT04gbmV0d29yayB0byBFdmVyc2NhbGUuIFRoZSBuZXcgbmFtaW5nIHJlZmxlY3RzIGEgbmWPAf53IHN0YWdlIGluIHRoZSBwcm90b2NvbCBkZXZlbG9wbWVudCBhbmQgYmV0dGVyIHJlZmxlY3RzIGl0cyBwb3NpdGlvbmluZy5cblxuV2UgcHJvcG9zZSB0byBzdXBwb3J0IHRoZSBibG9ja2NoYWluIHJlYnJhbmRpbmcgYW5kkAH+IHVwZGF0ZSB0aGUgY29ycmVzcG9uZGluZyBuYW1lcyBpbiB0aGUgaW50ZXJmYWNlOlxuKiBOZXR3b3JrIG5hbWU6IGZyb20gRnJlZVRPTiB0byBFdmVyc2NhbGVcbiogQ29pbiBuYW1lOiBmcm9tIFRPTiBDcnlzdGFsIHRvIJEADsOKdmVyIl0BZdAAAAAAAAAAAAAAAAAdzWUAQAAKZmYNhs5LApttCjY5A1u0utSERpDI3L+CUpwwJm1QaJMBCAMFmxiUAB5SZW5hbWUgYWNjZXB0ZWQ=").trust_me();

        if let Some(events) = extract_events(
            &transaction,
            transaction.tx_hash().trust_me(),
            &all_events.dao_root,
        ) {
            let (_, events) = split(events.output);
            for event in events {
                if event.function_name.as_str() == "ProposalCreated" {
                    let data: ProposalCreated = event.input.unpack().trust_me();

                    let ton_action: ProposalTonAction = data
                        .ton_actions
                        .first()
                        .map(|x| x.clone())
                        .trust_me()
                        .try_into()
                        .trust_me();

                    assert_eq!(ton_action.value, "1000000000");
                    assert_eq!(
                        ton_action.target,
                        "0:00a66660d86ce4b029b6d0a3639035bb4bad4844690c8dcbf82529c30266d506"
                    );
                    assert_eq!(
                        ton_action.payload,
                        "te6ccgEBAgEAGAABCAMFmxgBAB5SZW5hbWUgYWNjZXB0ZWQ="
                    );
                }
            }
        }
    }
}
