use super::Context;

use crate::api::requests::*;
use crate::api::responses::*;
use crate::api::utils::*;

pub async fn post_votes_search(
    ctx: Context,
    input: VotesRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(reply_sequence(
        ctx.services
            .search_votes(input.into())
            .await
            .map_err(BadRequestError)?
            .map(VoteResponse::from),
    ))
}
