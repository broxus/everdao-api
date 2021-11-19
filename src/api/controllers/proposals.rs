use futures::prelude::future::*;

use super::Context;
use crate::api::requests::{
    GraphRequest, SearchProposalsRequest, SearchTransactionsRequest, SearchTransfersRequest,
    UserPageStakingRequest,
};
use crate::api::responses::{
    GraphDataResponse, ProposalsResponse, TransactionsTableResponse, TransfersTableResponse,
};
use crate::api::utils::*;

pub fn post_search_proposals(
    ctx: Context,
    input: SearchProposalsRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let (user_balances, total_count) = ctx
            .services
            .search_proposals(input.clone())
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = ProposalsResponse::from((user_balances, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn search_transfers(
    ctx: Context,
    input: SearchTransfersRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let (user_balances, total_count) = ctx
            .services
            .search_transfers(input.clone())
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = TransfersTableResponse::from((user_balances, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn search_transactions(
    ctx: Context,
    input: SearchTransactionsRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let (transactions, total_count) = ctx
            .services
            .search_transactions(input.clone())
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = TransactionsTableResponse::from((transactions, total_count));

        Ok(warp::reply::json(&(res)))
    }
    .boxed()
}

pub fn post_graph_tvl(
    ctx: Context,
    input: GraphRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let graph_data = ctx.services.post_graph(input).await;

        let res = graph_data
            .into_iter()
            .map(|x| GraphDataResponse {
                data: x.balance,
                timestamp: x.timestamp as i64 * 1000,
            })
            .collect::<Vec<_>>();

        Ok(warp::reply::json(&(res)))
    }
    .boxed()
}

pub fn post_graph_apr(
    ctx: Context,
    input: GraphRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let graph_data = ctx.services.post_graph(input).await;

        let res = graph_data
            .into_iter()
            .map(|x| GraphDataResponse {
                data: x.apr,
                timestamp: x.timestamp as i64 * 1000,
            })
            .collect::<Vec<_>>();

        Ok(warp::reply::json(&(res)))
    }
    .boxed()
}

pub fn get_main_staking(
    ctx: Context,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let res = ctx.services.get_main_page_staking().await;
        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn post_user_staking(
    ctx: Context,
    input: UserPageStakingRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let res = ctx.services.post_user_page_staking(input).await;
        Ok(warp::reply::json(&res))
    }
    .boxed()
}
