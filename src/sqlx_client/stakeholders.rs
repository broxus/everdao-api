use crate::api::requests::SearchStakeholdersRequest;
use crate::models::sqlx::UserBalanceFromDb;
use crate::models::stakeholders_ordering::StakeholdersOrdering;
use crate::sqlx_client::SqlxClient;
use itertools::Itertools;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::Row;

impl SqlxClient {
    pub async fn search_stakeholders(
        &self,
        input: SearchStakeholdersRequest,
    ) -> Result<(Vec<UserBalanceFromDb>, i32), anyhow::Error> {
        let (updates, args_len, args, mut args_clone) = filter_stakeholders_query(&input);

        let mut query = "SELECT user_address, user_kind, stake_balance, frozen_stake, last_reward, 
            total_reward, until_frozen, updated_at, created_at FROM user_balances"
            .to_string();
        if !updates.is_empty() {
            query = format!("{} WHERE {}", query, updates.iter().format(" AND "));
        }

        let mut query_count = "SELECT COUNT(*) FROM user_balances".to_string();
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
                StakeholdersOrdering::UpdateAtAscending => "ORDER BY updated_at",
                StakeholdersOrdering::UpdateAtDescending => "ORDER BY updated_at DESC",
                StakeholdersOrdering::StakeAscending => "ORDER BY stake_balance",
                StakeholdersOrdering::StakeDescending => "ORDER BY stake_balance DESC",
                StakeholdersOrdering::FrozenStakeAscending => "ORDER BY frozen_stake",
                StakeholdersOrdering::FrozenStakeDescending => "ORDER BY frozen_stake DESC",
                StakeholdersOrdering::LastRewardAscending => "ORDER BY last_reward",
                StakeholdersOrdering::LastRewardDescending => "ORDER BY last_reward DESC",
                StakeholdersOrdering::TotalRewardAscending => "ORDER BY total_reward",
                StakeholdersOrdering::TotalRewardDescending => "ORDER BY total_reward DESC",
                StakeholdersOrdering::CreatedAtAscending => "ORDER BY created_at",
                StakeholdersOrdering::CreatedAtDescending => "ORDER BY created_at DESC",
            }
        } else {
            "ORDER BY updated_at DESC"
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
            .map(|x| UserBalanceFromDb {
                user_address: x.get(0),
                user_kind: x.get(1),
                stake_balance: x.get(2),
                frozen_stake: x.get(3),
                last_reward: x.get(4),
                total_reward: x.get(5),
                until_frozen: x.get(6),
                updated_at: x.get(7),
                created_at: x.get(8),
            })
            .collect::<Vec<_>>();

        Ok((res, total_count))
    }
}

pub fn filter_stakeholders_query(
    input: &SearchStakeholdersRequest,
) -> (Vec<String>, i32, PgArguments, PgArguments) {
    let SearchStakeholdersRequest {
        user_balance_ge,
        user_balance_le,
        stakeholder_kind,
        until_frozen_ge,
        until_frozen_le,
        last_reward_ge,
        last_reward_le,
        total_reward_ge,
        total_reward_le,
        frozen_stake_ge,
        frozen_stake_le,
        created_at_ge,
        created_at_le,
        ..
    } = input.clone();

    let mut args = PgArguments::default();
    let mut args_clone = PgArguments::default();
    let mut updates = Vec::new();
    let mut args_len = 0;

    if let Some(user_balance_ge) = user_balance_ge {
        updates.push(format!("stake_balance >= ${}", args_len + 1,));
        args_len += 1;
        args.add(user_balance_ge);
        args_clone.add(user_balance_ge);
    }

    if let Some(user_balance_le) = user_balance_le {
        updates.push(format!("stake_balance <= ${}", args_len + 1,));
        args_len += 1;
        args.add(user_balance_le);
        args_clone.add(user_balance_le)
    }

    if let Some(stakeholder_kind) = stakeholder_kind {
        updates.push(format!("user_kind = ${}", args_len + 1,));
        args_len += 1;
        args.add(stakeholder_kind.to_string());
        args_clone.add(stakeholder_kind.to_string())
    }

    if let Some(until_frozen_ge) = until_frozen_ge {
        updates.push(format!("until_frozen >= ${}", args_len + 1,));
        args_len += 1;
        args.add(until_frozen_ge);
        args_clone.add(until_frozen_ge)
    }

    if let Some(until_frozen_le) = until_frozen_le {
        updates.push(format!("until_frozen <= ${}", args_len + 1,));
        args_len += 1;
        args.add(until_frozen_le);
        args_clone.add(until_frozen_le)
    }

    if let Some(last_reward_ge) = last_reward_ge {
        updates.push(format!("last_reward >= ${}", args_len + 1,));
        args_len += 1;
        args.add(last_reward_ge);
        args_clone.add(last_reward_ge)
    }

    if let Some(last_reward_le) = last_reward_le {
        updates.push(format!("last_reward <= ${}", args_len + 1,));
        args_len += 1;
        args.add(last_reward_le);
        args_clone.add(last_reward_le)
    }

    if let Some(total_reward_ge) = total_reward_ge {
        updates.push(format!("total_reward >= ${}", args_len + 1,));
        args_len += 1;
        args.add(total_reward_ge);
        args_clone.add(total_reward_ge)
    }

    if let Some(total_reward_le) = total_reward_le {
        updates.push(format!("total_reward <= ${}", args_len + 1,));
        args_len += 1;
        args.add(total_reward_le);
        args_clone.add(total_reward_le)
    }

    if let Some(frozen_stake_ge) = frozen_stake_ge {
        updates.push(format!("frozen_stake >= ${}", args_len + 1,));
        args_len += 1;
        args.add(frozen_stake_ge);
        args_clone.add(frozen_stake_ge)
    }

    if let Some(frozen_stake_le) = frozen_stake_le {
        updates.push(format!("frozen_stake <= ${}", args_len + 1,));
        args_len += 1;
        args.add(frozen_stake_le);
        args_clone.add(frozen_stake_le)
    }

    if let Some(created_at_ge) = created_at_ge {
        updates.push(format!("created_at >= ${}", args_len + 1,));
        args_len += 1;
        args.add(created_at_ge);
        args_clone.add(created_at_ge)
    }

    if let Some(created_at_le) = created_at_le {
        updates.push(format!("created_at <= ${}", args_len + 1,));
        args_len += 1;
        args.add(created_at_le);
        args_clone.add(created_at_le)
    }

    (updates, args_len, args, args_clone)
}
