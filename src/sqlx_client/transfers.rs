use std::collections::HashMap;

use itertools::Itertools;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::Row;

use crate::api::requests::SearchTransfersRequest;
use crate::models::sqlx::TransferFromDb;
use crate::models::transfers_ordering::TransfersOrdering;
use crate::sqlx_client::SqlxClient;

impl SqlxClient {
    pub async fn new_ton_eth_transfer(
        &self,
        transfer: TransferFromDb,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(r#"INSERT INTO transfers (ton_message_hash, ton_transaction_hash, contract_address, event_index, user_address,
                       volume_exec, ton_token_address, eth_token_address, transfer_kind, status, required_votes, confirm_votes, reject_votes,
                       timestamp_block_updated_at, timestamp_block_created_at, chain_id, burn_callback_timestamp_lt)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"#,
        transfer.ton_message_hash,
        transfer.ton_transaction_hash,
        transfer.contract_address,
        transfer.event_index,
        transfer.user_address,
        transfer.volume_exec,
        transfer.ton_token_address,
        transfer.eth_token_address,
        transfer.transfer_kind,
        transfer.status,
        transfer.required_votes,
        transfer.confirm_votes,
        transfer.reject_votes,
        transfer.timestamp_block_updated_at,
        transfer.timestamp_block_created_at,
        transfer.chain_id,
        transfer.burn_callback_timestamp_lt).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn new_eth_ton_transfer(
        &self,
        transfer: TransferFromDb,
    ) -> Result<(), anyhow::Error> {
        if sqlx::query!(r#"INSERT INTO transfers (ton_message_hash, ton_transaction_hash, contract_address, event_index, user_address,
                       volume_exec, ton_token_address, eth_token_address, transfer_kind, status, required_votes, confirm_votes, reject_votes, 
                       timestamp_block_updated_at, timestamp_block_created_at, graphql_timestamp, chain_id, eth_transaction_hash)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
            transfer.ton_message_hash,
            transfer.ton_transaction_hash,
            transfer.contract_address,
            transfer.event_index,
            transfer.user_address,
            transfer.volume_exec,
            transfer.ton_token_address,
            transfer.eth_token_address,
            transfer.transfer_kind,
            transfer.status,
            transfer.required_votes,
            transfer.confirm_votes,
            transfer.reject_votes,
            transfer.timestamp_block_updated_at,
            transfer.timestamp_block_created_at,
            transfer.graphql_timestamp,
            transfer.chain_id,
            transfer.eth_transaction_hash).execute(&self.pool).await.is_err() {
            match transfer.graphql_timestamp {
                None => {
                    sqlx::query!(r#"UPDATE transfers SET ton_message_hash = $1, ton_transaction_hash = $2, contract_address = $3, timestamp_block_updated_at = $4 WHERE event_index = $5 and eth_transaction_hash = $6"#,
                transfer.ton_message_hash,
                transfer.ton_transaction_hash,
                transfer.contract_address,
                transfer.timestamp_block_created_at,
                transfer.event_index,
                transfer.eth_transaction_hash).execute(&self.pool).await?;
                }
                Some(_) => {
                    sqlx::query!(r#"UPDATE transfers SET ton_message_hash = $1, ton_transaction_hash = $2, contract_address = $3, timestamp_block_updated_at = $4, graphql_timestamp = $6 WHERE event_index = $5 and eth_transaction_hash = $7"#,
                transfer.ton_message_hash,
                transfer.ton_transaction_hash,
                transfer.contract_address,
                transfer.timestamp_block_created_at,
                transfer.event_index,
                transfer.graphql_timestamp,
                transfer.eth_transaction_hash).execute(&self.pool).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn get_transfer_by_contract_address(
        &self,
        contract_address: &str,
    ) -> Result<TransferFromDb, anyhow::Error> {
        sqlx::query_as!(
            TransferFromDb,
            r#"SELECT * FROM transfers WHERE contract_address = $1"#,
            contract_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn update_required_votes(
        &self,
        contract_address: &str,
        required_votes: i32,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"UPDATE transfers SET required_votes = $1 WHERE contract_address = $2"#,
            required_votes,
            contract_address
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn search_transfers(
        &self,
        input: SearchTransfersRequest,
    ) -> Result<(Vec<TransferFromDb>, i64), anyhow::Error> {
        let (updates, args_len, args, mut args_clone) = filter_transfers_query(&input);

        let mut query = "SELECT ton_message_hash, ton_transaction_hash, contract_address, event_index, user_address, 
            volume_exec, ton_token_address, eth_token_address, transfer_kind, status, required_votes, confirm_votes, reject_votes, timestamp_block_updated_at, 
            timestamp_block_created_at, graphql_timestamp, chain_id, updated_at, created_at, eth_transaction_hash FROM transfers"
            .to_string();
        if !updates.is_empty() {
            query = format!("{} WHERE {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = "SELECT COUNT(*) FROM transfers".to_string();
        if !updates.is_empty() {
            query_count = format!("{} WHERE {}", query_count, updates.iter().format(" AND "));
        }

        let total_count: i64 = sqlx::query_with(&query_count, args)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.get(0))
            .unwrap_or_default();

        let ordering = if let Some(ordering) = input.ordering {
            match ordering {
                TransfersOrdering::VolumeExecAscending => "ORDER BY volume_exec",
                TransfersOrdering::VolumeExecDescending => "ORDER BY volume_exec DESC",
                TransfersOrdering::UpdateAtAscending => "ORDER BY timestamp_block_updated_at",
                TransfersOrdering::UpdateAtDescending => "ORDER BY timestamp_block_updated_at DESC",
                TransfersOrdering::CreatedAtAscending => "ORDER BY timestamp_block_created_at",
                TransfersOrdering::CreatedAtDescending => {
                    "ORDER BY timestamp_block_created_at DESC"
                }
            }
        } else {
            "ORDER BY timestamp_block_created_at DESC"
        };

        query = format!(
            "{} {} OFFSET ${} LIMIT ${}",
            query,
            ordering,
            args_len + 1,
            args_len + 2
        );

        args_clone.add(input.offset);
        args_clone.add(input.limit);

        let transactions = sqlx::query_with(&query, args_clone)
            .fetch_all(&self.pool)
            .await?;

        let res = transactions
            .into_iter()
            .map(|x| TransferFromDb {
                ton_message_hash: x.get(0),
                ton_transaction_hash: x.get(1),
                contract_address: x.get(2),
                event_index: x.get(3),
                eth_transaction_hash: x.get(19),
                user_address: x.get(4),
                volume_exec: x.get(5),
                ton_token_address: x.get(6),
                eth_token_address: x.get(7),
                transfer_kind: x.get(8),
                status: x.get(9),
                required_votes: x.get(10),
                confirm_votes: x.get(11),
                reject_votes: x.get(12),
                burn_callback_timestamp_lt: None,
                timestamp_block_updated_at: x.get(13),
                timestamp_block_created_at: x.get(14),
                graphql_timestamp: x.get(15),
                chain_id: x.get(16),
                updated_at: x.get(17),
                created_at: x.get(18),
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }

    pub async fn get_last_graphql_timestamp(&self) -> Result<HashMap<i32, i32>, anyhow::Error> {
        sqlx::query!(
            r#"SELECT chain_id, max(graphql_timestamp) FROM transfers WHERE event_index IS NOT NULL GROUP BY chain_id"#
        )
            .fetch_all(&self.pool)
            .await
            .map(|x| x.into_iter().map(|x| (x.chain_id, x.max.unwrap_or_default())).collect::<HashMap<_, _>>())
            .map_err(anyhow::Error::new)
    }
}

fn filter_transfers_query(
    input: &SearchTransfersRequest,
) -> (Vec<String>, i32, PgArguments, PgArguments) {
    let SearchTransfersRequest {
        user_address,
        ton_token_address,
        eth_token_address,
        volume_exec_ge,
        volume_exec_le,
        status,
        updated_at_ge,
        updated_at_le,
        created_at_ge,
        created_at_le,
        chain_id,
        transaction_hash,
        ..
    } = input.clone();

    let mut args = PgArguments::default();
    let mut args_clone = PgArguments::default();
    let mut updates = Vec::new();
    let mut args_len = 0;

    if let Some(user_address) = user_address {
        updates.push(format!("user_address = ${}", args_len + 1,));
        args_len += 1;
        args.add(user_address.clone());
        args_clone.add(user_address);
    }

    if let Some(ton_token_address) = ton_token_address {
        updates.push(format!("ton_token_address = ${}", args_len + 1,));
        args_len += 1;
        args.add(ton_token_address.clone());
        args_clone.add(ton_token_address);
    }

    if let Some(eth_token_address) = eth_token_address {
        updates.push(format!("eth_token_address = ${}", args_len + 1,));
        args_len += 1;
        args.add(eth_token_address.clone());
        args_clone.add(eth_token_address);
    }

    if let Some(volume_exec_ge) = volume_exec_ge {
        updates.push(format!("volume_exec >= ${}", args_len + 1,));
        args_len += 1;
        args.add(volume_exec_ge);
        args_clone.add(volume_exec_ge);
    }

    if let Some(volume_exec_le) = volume_exec_le {
        updates.push(format!("volume_exec <= ${}", args_len + 1,));
        args_len += 1;
        args.add(volume_exec_le);
        args_clone.add(volume_exec_le);
    }

    if let Some(chain_id) = chain_id {
        updates.push(format!("chain_id = ${}", args_len + 1,));
        args_len += 1;
        args.add(chain_id);
        args_clone.add(chain_id);
    }

    if let Some(transaction_hash) = transaction_hash {
        let transaction_hash = hex::decode(transaction_hash).unwrap_or_default();
        updates.push(format!("transaction_hash = ${}", args_len + 1,));
        args_len += 1;
        args.add(transaction_hash.clone());
        args_clone.add(transaction_hash);
    }

    if let Some(status) = status {
        let status = status.to_string();
        updates.push(format!("status = ${}", args_len + 1,));
        args_len += 1;
        args.add(status.clone());
        args_clone.add(status);
    }

    if let Some(updated_at_ge) = updated_at_ge {
        let updated_at_ge = (updated_at_ge / 1000) as i32;
        updates.push(format!("timestamp_block_updated_at >= ${}", args_len + 1,));
        args_len += 1;
        args.add(updated_at_ge);
        args_clone.add(updated_at_ge);
    }

    if let Some(updated_at_le) = updated_at_le {
        let updated_at_le = (updated_at_le / 1000) as i32;
        updates.push(format!("timestamp_block_updated_at <= ${}", args_len + 1,));
        args_len += 1;
        args.add(updated_at_le);
        args_clone.add(updated_at_le);
    }

    if let Some(created_at_ge) = created_at_ge {
        let created_at_ge = (created_at_ge / 1000) as i32;
        updates.push(format!("timestamp_block_created_at >= ${}", args_len + 1,));
        args_len += 1;
        args.add(created_at_ge);
        args_clone.add(created_at_ge);
    }

    if let Some(created_at_le) = created_at_le {
        let created_at_le = (created_at_le / 1000) as i32;
        updates.push(format!("timestamp_block_created_at <= ${}", args_len + 1,));
        args_len += 1;
        args.add(created_at_le);
        args_clone.add(created_at_le);
    }

    (updates, args_len, args, args_clone)
}

#[cfg(test)]
mod test {
    use crate::api::requests::SearchTransfersRequest;
    use crate::models::transfer_status::TransferStatus;
    use crate::models::transfers_ordering::TransfersOrdering;
    use crate::sqlx_client::SqlxClient;
    use sqlx::types::Decimal;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_transfer_search() {
        let pg_pool =
            PgPool::connect("postgresql://postgres:postgres@localhost:5432/bridge_dao_indexer")
                .await
                .unwrap();
        let sqlx_client = SqlxClient::new(pg_pool);
        let input = SearchTransfersRequest {
            transaction_hash: None,
            user_address: Some("test".to_string()),
            ton_token_address: Some("test".to_string()),
            eth_token_address: Some("test".to_string()),
            limit: 10,
            offset: 1,
            volume_exec_ge: Some(Decimal::new(1, 0)),
            volume_exec_le: Some(Decimal::new(1, 0)),
            chain_id: Some(1),
            status: Some(TransferStatus::Confirmed),
            updated_at_ge: Some(100),
            updated_at_le: Some(100),
            created_at_ge: Some(100),
            created_at_le: Some(100),
            ordering: Some(TransfersOrdering::CreatedAtDescending),
        };
        sqlx_client.search_transfers(input).await.unwrap();
    }
}
