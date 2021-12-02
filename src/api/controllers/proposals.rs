use futures::prelude::future::*;

use crate::api::filters::proposals;

use super::Context;
use crate::api::requests::{SearchProposalVotesRequest, SearchVotesRequest};
use crate::api::responses::{ProposalsResponse, VotesResponse};
use crate::api::utils::*;
use crate::models::SearchProposalsRequest;

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
    address: String,
    ctx: Context,
    input: SearchProposalVotesRequest,
) -> BoxFuture<'static, Result<impl warp::Reply, warp::Rejection>> {
    async move {
        let mut search = SearchVotesRequest::from(input);
        search.proposal_address = Some(address);
        let (votes, total_count) = ctx
            .services
            .search_votes(search)
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
        input.voter_address = Some(address);
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
        input.voter_address = Some(address);
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
