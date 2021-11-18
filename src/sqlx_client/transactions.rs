use crate::api::requests::SearchTransactionsRequest;
use crate::models::sqlx::TransactionFromDb;
use crate::sqlx_client::SqlxClient;

use crate::models::transaction_ordering::TransactionOrdering;
use itertools::Itertools;
use sqlx::postgres::PgArguments;
use sqlx::Row;
use sqlx::{Arguments, Postgres, Transaction};

impl SqlxClient {
    pub async fn count_transaction(&self) -> i64 {
        sqlx::query!(r#"SELECT COUNT(*) FROM transactions"#)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.count.unwrap_or_default())
            .unwrap_or_default()
    }
    pub async fn search_transactions(
        &self,
        input: SearchTransactionsRequest,
    ) -> Result<(Vec<TransactionFromDb>, i32), anyhow::Error> {
        let (updates, args_len, args, mut args_clone) = filter_transactions_query(&input);

        let mut query = "SELECT message_hash, transaction_hash, transaction_kind, user_address, user_public_key, 
            bridge_exec, timestamp_block, created_at FROM transactions"
            .to_string();
        if !updates.is_empty() {
            query = format!("{} WHERE {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = "SELECT COUNT(*) FROM transactions".to_string();
        if !updates.is_empty() {
            query_count = format!("{} WHERE {}", query_count, updates.iter().format(" AND "));
        }

        let total_count: i32 = sqlx::query_with(&query_count, args)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.get(0))
            .unwrap_or_default();

        let ordering = if let Some(ordering) = input.ordering {
            match ordering {
                TransactionOrdering::AmountAscending => "ORDER BY bridge_exec",
                TransactionOrdering::AmountDescending => "ORDER BY bridge_exec DESC",
                TransactionOrdering::TimestampBlockAscending => "ORDER BY timestamp_block",
                TransactionOrdering::TimestampBlockAtDescending => "ORDER BY timestamp_block DESC",
            }
        } else {
            "ORDER BY timestamp_block DESC"
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
            .map(|x| TransactionFromDb {
                message_hash: x.get(0),
                transaction_hash: x.get(1),
                transaction_kind: x.get(2),
                user_address: x.get(3),
                user_public_key: x.get(4),
                bridge_exec: x.get(5),
                timestamp_block: x.get(6),
                created_at: x.get(7),
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }
}

pub fn filter_transactions_query(
    input: &SearchTransactionsRequest,
) -> (Vec<String>, i32, PgArguments, PgArguments) {
    let SearchTransactionsRequest {
        transaction_kind,
        amount_ge,
        amount_le,
        timestamp_block_ge,
        timestamp_block_le,
        ..
    } = input.clone();

    let mut args = PgArguments::default();
    let mut args_clone = PgArguments::default();
    let mut updates = Vec::new();
    let mut args_len = 0;

    if let Some(transaction_kind) = transaction_kind {
        updates.push(format!("transaction_kind = ${}", args_len + 1,));
        args_len += 1;
        args.add(transaction_kind.to_string());
        args_clone.add(transaction_kind.to_string());
    }

    if let Some(amount_ge) = amount_ge {
        updates.push(format!("bridge_exec >= ${}", args_len + 1,));
        args_len += 1;
        args.add(amount_ge);
        args_clone.add(amount_ge);
    }

    if let Some(amount_le) = amount_le {
        updates.push(format!("bridge_exec <= ${}", args_len + 1,));
        args_len += 1;
        args.add(amount_le);
        args_clone.add(amount_le);
    }

    if let Some(timestamp_block_ge) = timestamp_block_ge {
        updates.push(format!("timestamp_block >= ${}", args_len + 1,));
        args_len += 1;
        args.add(timestamp_block_ge);
        args_clone.add(timestamp_block_ge);
    }

    if let Some(timestamp_block_le) = timestamp_block_le {
        updates.push(format!("timestamp_block <= ${}", args_len + 1,));
        args_len += 1;
        args.add(timestamp_block_le);
        args_clone.add(timestamp_block_le);
    }

    (updates, args_len, args, args_clone)
}

pub async fn new_transaction(
    transaction: TransactionFromDb,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<i64, anyhow::Error> {
    let created_at: i64 = sqlx::query!(r#"INSERT INTO transactions (message_hash, transaction_hash, 
                          transaction_kind, user_address, user_public_key, bridge_exec, timestamp_block) 
                          VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING created_at"#,
        transaction.message_hash,
        transaction.transaction_hash,
        transaction.transaction_kind,
        transaction.user_address,
        transaction.user_public_key,
        transaction.bridge_exec,
        transaction.timestamp_block).fetch_one(tx).await.map(|x| x.created_at)?;
    Ok(created_at)
}
