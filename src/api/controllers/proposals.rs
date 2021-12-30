use super::Context;

use crate::api::requests::*;
use crate::api::responses::*;
use crate::api::utils::*;

pub async fn post_proposals_search(
    ctx: Context,
    input: ProposalsRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (proposals, total_count) = ctx
        .services
        .search_proposals(input.into())
        .await
        .map_err(BadRequestError)?;

    Ok(warp::reply::json(&ProposalsResponse {
        proposals: proposals.map(ProposalResponse::from).collect::<Vec<_>>(),
        total_count,
    }))
}

pub async fn get_proposals_overview(ctx: Context) -> Result<impl warp::Reply, warp::Rejection> {
    let overview = ctx.services.overview().await.map_err(BadRequestError)?;

    Ok(warp::reply::json(&overview))
}
