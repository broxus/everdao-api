use crate::api::requests::{
    GraphRequest, SearchStakeholdersRequest, SearchTransactionsRequest, UserPageStakingRequest,
};
use crate::api::responses::{MainPageStakingResponse, UserPageStakingResponse};
use crate::models::sqlx::{GraphDataFromDb, TransactionFromDb, UserBalanceFromDb};
use crate::services::utils::DAY_SEC;
use crate::services::Services;
use crate::sqlx_client::graph_data::fill_graph;
use chrono::Utc;
use nekoton_abi::num_traits::{One, Zero};
use sqlx::types::Decimal;

impl Services {
    pub async fn search_stakeholders(
        &self,
        input: SearchStakeholdersRequest,
    ) -> Result<(Vec<UserBalanceFromDb>, i32), anyhow::Error> {
        self.sqlx_client.search_stakeholders(input).await
    }

    pub async fn search_transactions(
        &self,
        input: SearchTransactionsRequest,
    ) -> Result<(Vec<TransactionFromDb>, i32), anyhow::Error> {
        self.sqlx_client.search_transactions(input).await
    }

    pub async fn post_graph(&self, input: GraphRequest) -> Vec<GraphDataFromDb> {
        let graph_data = self
            .sqlx_client
            .get_graph_data(input.from, input.to, input.timeframe.to_string())
            .await
            .unwrap_or_default();
        fill_graph(self.sqlx_client.clone(), graph_data, input).await
    }

    pub async fn get_main_page_staking(&self) -> MainPageStakingResponse {
        let last_bridge_balance = self.sqlx_client.get_last_bride_balance().await.unwrap();
        let timestamp_1d_ago = Utc::now().timestamp() as i32 - DAY_SEC;
        let tvl_1d_ago = self
            .sqlx_client
            .get_bridge_tvl_by_timestamp(timestamp_1d_ago)
            .await
            .unwrap_or_default();
        MainPageStakingResponse {
            tvl: last_bridge_balance.bridge_balance,
            tvl_change: match tvl_1d_ago.is_zero() {
                true => Decimal::zero(),
                false => {
                    (last_bridge_balance.bridge_balance / tvl_1d_ago).round_dp(4)
                        * Decimal::new(100, 0)
                        - Decimal::one()
                }
            },
            reward_30d: Default::default(),        // todo!
            reward_30d_change: Default::default(), // todo!
            average_apr: last_bridge_balance.average_apr,
            stakeholders: last_bridge_balance.stakeholders as i64,
        }
    }

    pub async fn post_user_page_staking(
        &self,
        input: UserPageStakingRequest,
    ) -> UserPageStakingResponse {
        let last_user_balance = self
            .sqlx_client
            .get_stakeholder(&input.user_address)
            .await
            .unwrap_or_default();
        let timestamp_1d_ago = Utc::now().timestamp() as i32 - DAY_SEC;
        let stake_balance_1d_ago = self
            .sqlx_client
            .get_user_balance_by_timestamp(&input.user_address, timestamp_1d_ago)
            .await
            .unwrap_or_default();
        UserPageStakingResponse {
            user_tvl: last_user_balance.stake_balance,
            user_tvl_change: match stake_balance_1d_ago.is_zero() {
                true => Decimal::zero(),
                false => {
                    (last_user_balance.stake_balance / stake_balance_1d_ago).round_dp(4)
                        * Decimal::new(100, 0)
                        - Decimal::one()
                }
            },
            user_frozen_stake: last_user_balance.frozen_stake,
            user_30d_reward: last_user_balance.last_reward, // todo!
            user_30d_reward_change: Default::default(),     // todo!
            average_apr: Default::default(),                // todo!
        }
    }
}
