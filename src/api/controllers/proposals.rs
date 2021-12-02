use futures::prelude::future::*;

use super::Context;
use crate::api::responses::{ProposalsResponse, VotesResponse};
use crate::api::utils::*;
use crate::models::{SearchProposalsRequest, SearchVotesRequest};

pub fn post_search_proposals(
    ctx: Context,
    input: SearchProposalsRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let (proposals, total_count) = ctx
            .services
            .search_proposals(input)
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = ProposalsResponse::from((proposals, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn post_proposal_votes(
    proposal_id: i32,
    ctx: Context,
    mut input: SearchVotesRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        input.proposal_id = Some(proposal_id);
        let (votes, total_count) = ctx
            .services
            .search_votes(input)
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = VotesResponse::from((votes, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn post_voter_votes(
    address: String,
    ctx: Context,
    mut input: SearchVotesRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        input.voter = Some(address);
        let (votes, total_count) = ctx
            .services
            .search_votes(input)
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = VotesResponse::from((votes, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn post_voters_votes(
    ctx: Context,
    input: SearchVotesRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let (votes, total_count) = ctx
            .services
            .search_votes(input)
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = VotesResponse::from((votes, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}

pub fn post_voters_proposals(
    address: String,
    ctx: Context,
    mut input: SearchProposalsRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        input.proposer = Some(address);
        let (proposals, total_count) = ctx
            .services
            .search_proposals(input)
            .await
            .map_err(|e| warp::reject::custom(BadRequestError { 0: e.to_string() }))?;

        let res = ProposalsResponse::from((proposals, total_count));

        Ok(warp::reply::json(&res))
    }
    .boxed()
}
