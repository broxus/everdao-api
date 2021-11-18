use crate::api::requests::GraphRequest;
use crate::models::sqlx::GraphDataFromDb;
use crate::models::timeframe::Timeframe;
use crate::services::utils::HOUR_MS;
use crate::sqlx_client::SqlxClient;
use chrono::{NaiveDateTime, Timelike, Utc};
use itertools::Itertools;
use std::collections::HashMap;

impl SqlxClient {
    pub async fn get_graph_data(
        &self,
        from: i64,
        to: i64,
        timeframe: String,
    ) -> Result<Vec<GraphDataFromDb>, anyhow::Error> {
        sqlx::query_as!(
            GraphDataFromDb,
            r#"SELECT * FROM graph_data WHERE timestamp >= $1 AND timestamp < $2 AND kind = $3 ORDER BY timestamp"#,
            from,
            to,
            &timeframe.to_string(),
        ).fetch_all(&self.pool).await.map_err(anyhow::Error::new)
    }

    pub async fn get_graph_data_by_timestamp(
        &self,
        timestamp: i64,
        kind: String,
    ) -> Result<GraphDataFromDb, anyhow::Error> {
        sqlx::query_as!(
            GraphDataFromDb,
            r#"SELECT * FROM graph_data WHERE timestamp < $1 AND kind = $2 ORDER BY timestamp DESC LIMIT 1"#,
            timestamp,
            &kind,
        ).fetch_one(&self.pool).await.map_err(anyhow::Error::new)
    }
}

pub async fn fill_graph(
    sqlx_client: SqlxClient,
    graph: Vec<GraphDataFromDb>,
    input: GraphRequest,
) -> Vec<GraphDataFromDb> {
    let GraphRequest {
        from,
        to,
        timeframe: kind,
    } = input;
    let date_time_now = Utc::now();
    let timestamp_now = date_time_now.timestamp_millis();
    let date_time_from = NaiveDateTime::from_timestamp(from / 1000, 0);
    let (time_skip, mut date_time_from) = match kind {
        Timeframe::H1 => (
            HOUR_MS,
            date_time_from
                .date()
                .and_hms(date_time_from.hour(), 0, 0)
                .timestamp_millis(),
        ),
        Timeframe::D1 => (
            HOUR_MS * 24,
            date_time_from.date().and_hms(0, 0, 0).timestamp_millis(),
        ),
    };

    let mut timestamps = vec![];
    while date_time_from < to && date_time_from < timestamp_now {
        timestamps.push(date_time_from);
        date_time_from += time_skip;
    }

    let mut graphs = graph
        .into_iter()
        .map(|x| (x.timestamp, x))
        .collect::<HashMap<_, _>>();
    if timestamps.len() == 1 {
        let time = *timestamps.get(0).unwrap();
        if let Some(x) = graphs.remove(&time) {
            return vec![x];
        }

        return match sqlx_client
            .get_graph_data_by_timestamp(time - 1, kind.to_string())
            .await
        {
            Ok(ok) => {
                vec![GraphDataFromDb {
                    kind: kind.to_string(),
                    apr: ok.apr,
                    balance: ok.balance,
                    timestamp: time,
                    reward: ok.reward,
                }]
            }
            Err(_) => {
                vec![]
            }
        };
    }

    for time in timestamps.windows(2) {
        let prev = *time.get(0).unwrap();
        let next = *time.get(1).unwrap();
        let prev_graph = match graphs.get(&prev) {
            None => match sqlx_client
                .get_graph_data_by_timestamp(prev - 1, kind.to_string())
                .await
            {
                Ok(mut ok) => {
                    ok.timestamp = prev;
                    ok
                }
                Err(_) => GraphDataFromDb {
                    kind: kind.to_string(),
                    apr: Default::default(),
                    balance: Default::default(),
                    timestamp: prev,
                    reward: Default::default(),
                },
            },
            Some(x) => x.clone(),
        };
        let next_graph = match graphs.get(&next) {
            None => GraphDataFromDb {
                kind: kind.to_string(),
                apr: prev_graph.apr,
                balance: prev_graph.balance,
                timestamp: next,
                reward: prev_graph.reward,
            },
            Some(x) => x.clone(),
        };

        if !prev_graph.apr.is_zero() {
            graphs.insert(prev, prev_graph);
        }

        if !next_graph.apr.is_zero() {
            graphs.insert(next, next_graph);
        }
    }

    graphs
        .into_iter()
        .map(|x| x.1)
        .sorted_by(|a, b| a.timestamp.cmp(&b.timestamp))
        .collect::<Vec<_>>()
}
