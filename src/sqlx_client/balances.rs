use crate::models::sqlx::{BridgeBalanceFromDb, TransactionFromDb, ProposalFromDb};
use crate::models::user_type::UserType;
use crate::sqlx_client::transactions::new_transaction;
use crate::sqlx_client::SqlxClient;
use sqlx::types::Decimal;

impl SqlxClient {
    pub async fn get_last_bride_balance(&self) -> Result<BridgeBalanceFromDb, anyhow::Error> {
        sqlx::query_as!(
            BridgeBalanceFromDb,
            r#"SELECT * FROM bridge_balances ORDER BY created_at DESC LIMIT 1"#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn get_last_user_info(
        &self,
        user_address: &str,
    ) -> Result<ProposalFromDb, anyhow::Error> {
        sqlx::query_as!(ProposalFromDb,
            r#"SELECT * FROM user_balances WHERE user_address = $1 ORDER BY created_at DESC LIMIT 1"#,
            user_address
        )
            .fetch_one(&self.pool)
            .await.map_err(anyhow::Error::new)
    }

    pub async fn get_stakeholder(
        &self,
        user_address: &str,
    ) -> Result<ProposalFromDb, anyhow::Error> {
        sqlx::query_as!(
            ProposalFromDb,
            r#"SELECT * FROM user_balances WHERE user_address = $1"#,
            user_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::new)
    }

    pub async fn get_user_balance_by_timestamp(
        &self,
        user_address: &str,
        timestamp: i32,
    ) -> Result<Decimal, anyhow::Error> {
        sqlx::query!(r#"SELECT user_balance FROM bridge_balances WHERE user_address = $1 AND timestamp_block <= $2 ORDER BY timestamp_block DESC limit 1"#, 
            user_address, timestamp).fetch_one(&self.pool).await.map(|x| x.user_balance).map_err(anyhow::Error::new)
    }

    pub async fn new_balance_transaction(
        &self,
        transaction: TransactionFromDb,
        bridge_balance: BridgeBalanceFromDb,
        user_balance: ProposalFromDb,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let created_at = new_transaction(transaction, &mut tx).await?;

        sqlx::query!(
            r#"INSERT INTO bridge_balances (message_hash, transaction_hash, transaction_kind,
                      user_address, user_balance, reward,
                      bridge_balance, stakeholders, average_apr, bridge_reward,
                      timestamp_block, created_at)
                      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
            bridge_balance.message_hash,
            bridge_balance.transaction_hash,
            bridge_balance.transaction_kind,
            bridge_balance.user_address,
            bridge_balance.user_balance,
            bridge_balance.reward,
            bridge_balance.bridge_balance,
            bridge_balance.stakeholders,
            bridge_balance.average_apr,
            bridge_balance.bridge_reward,
            bridge_balance.timestamp_block,
            created_at
        )
        .execute(&mut tx)
        .await?;

        sqlx::query!(r#"INSERT INTO user_balances (user_address, user_kind, stake_balance, frozen_stake, last_reward, total_reward, until_frozen, updated_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8) ON CONFLICT (user_address) DO UPDATE
        SET user_kind = $2, stake_balance = $3, last_reward = $5, total_reward = $6, updated_at = $8"#,
        user_balance.user_address,
        user_balance.user_kind,
        user_balance.stake_balance,
        user_balance.frozen_stake,
        user_balance.last_reward,
        user_balance.total_reward,
        user_balance.until_frozen,
        created_at).execute(&mut tx).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_bridge_tvl_by_timestamp(
        &self,
        timestamp: i32,
    ) -> Result<Decimal, anyhow::Error> {
        sqlx::query!(r#"SELECT bridge_balance FROM bridge_balances WHERE timestamp_block < $1 ORDER BY timestamp_block DESC LIMIT 1"#, timestamp).fetch_one(&self.pool).await.map(|x| x.bridge_balance).map_err(anyhow::Error::new)
    }

    pub async fn new_frozen_transaction(
        &self,
        user_address: &str,
        until_frozen: i32,
        frozen_stake: Decimal,
        transaction: TransactionFromDb,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;
        let created_at = new_transaction(transaction, &mut tx).await?;

        sqlx::query!(r#"INSERT INTO user_balances (user_address, user_kind, stake_balance, frozen_stake, last_reward, total_reward, until_frozen, updated_at, created_at)
        VALUES ($1, $2, 0, 0, 0, 0, $3, $4, $4) ON CONFLICT (user_address) DO UPDATE
        SET user_kind = $2, until_frozen = $3,  updated_at = $4, frozen_stake = $5"#,
        user_address,
        UserType::Relay.to_string(),
        until_frozen,
        created_at,
        frozen_stake).execute(&mut tx).await?;
        tx.commit().await?;

        Ok(())
    }

    pub async fn update_frozen_staking(&self, timestamp_now: i32) -> Result<(), anyhow::Error> {
        sqlx::query!(r#"UPDATE user_balances SET frozen_stake = stake_balance WHERE until_frozen >= $1 AND frozen_stake != user_balances.stake_balance"#, timestamp_now).execute(&self.pool).await?;
        sqlx::query!(r#"UPDATE user_balances SET frozen_stake = 0 WHERE until_frozen < $1 AND frozen_stake != 0"#, timestamp_now).execute(&self.pool).await?;
        Ok(())
    }
}
