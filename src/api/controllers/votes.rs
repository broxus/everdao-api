use super::Context;

use crate::api::requests::*;
use crate::api::responses::*;
use crate::api::utils::*;

pub async fn post_votes_search(
    ctx: Context,
    input: VotesRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (votes, total_count) = ctx
        .services
        .search_votes(input.into())
        .await
        .map_err(BadRequestError)?;

    Ok(warp::reply::json(&VotesResponse {
        votes: votes.map(VoteResponse::from).collect::<Vec<_>>(),
        total_count,
    }))
}
