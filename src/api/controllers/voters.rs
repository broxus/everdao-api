use super::Context;

use crate::api::requests::*;
use crate::api::responses::*;
use crate::api::utils::*;

pub async fn post_voters_search(
    address: String,
    ctx: Context,
    input: VotersRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (resp, total_count) = ctx
        .services
        .search_proposals_with_votes(address, input.into())
        .await
        .map_err(BadRequestError)?;

    Ok(warp::reply::json(&ProposalsWithVotesResponse {
        proposal_with_votes: resp
            .map(|(proposal, vote)| ProposalWithVoteResponse {
                vote: vote.into(),
                proposal: proposal.into(),
            })
            .collect::<Vec<_>>(),
        total_count,
    }))
}

pub async fn post_voters_proposal_count(
    ctx: Context,
    input: ProposalsCountRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resp = ctx
        .services
        .proposals_count(input.voters)
        .await
        .map_err(BadRequestError)?;

    Ok(warp::reply::json(
        &resp
            .into_iter()
            .map(|(voter, count)| ProposalCountResponse { voter, count })
            .collect::<Vec<_>>(),
    ))
}

pub async fn post_voters_proposal_count_search(
    ctx: Context,
    input: ProposalsCountSearchRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resp = ctx
        .services
        .search_proposals_count(input.into())
        .await
        .map_err(BadRequestError)?;

    Ok(warp::reply::json(
        &resp
            .into_iter()
            .map(|(voter, count)| ProposalCountResponse { voter, count })
            .collect::<Vec<_>>(),
    ))
}
