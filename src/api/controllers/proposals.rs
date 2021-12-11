use super::Context;

use crate::api::requests::*;
use crate::api::responses::*;
use crate::api::utils::*;

pub async fn post_proposals(
    ctx: Context,
    input: ProposalByIdRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(
        &ctx.services
            .get_proposal(input.id)
            .await
            .map_err(BadRequestError)?
            .map(ProposalResponse::from),
    ))
}

pub async fn post_proposals_search(
    ctx: Context,
    input: ProposalsRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(reply_sequence(
        ctx.services
            .search_proposals(input.into())
            .await
            .map_err(BadRequestError)?
            .map(ProposalResponse::from),
    ))
}

pub async fn post_voters_proposals(
    address: String,
    ctx: Context,
    input: ProposalsRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(reply_sequence(
        ctx.services
            .search_voter_proposals(address, input.into())
            .await
            .map_err(BadRequestError)?
            .map(ProposalResponse::from),
    ))
}
