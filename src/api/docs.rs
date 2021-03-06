#![allow(clippy::needless_update)]

use opg::*;

use crate::api::requests;
use crate::api::responses;
use crate::models::ProposalsOverview;

pub fn swagger(prod_url: &str, test_url: &str) -> String {
    let api = describe_api! {
        info: {
            title: "DAO",
            version: "1.0.0",
            description: r##"This API allows you to get the information of DAO"##,
        },
        servers: {
            prod_url,
            test_url
        },
        tags: {
            proposals,
            voters,
            votes,
        },
        paths: {
            ("proposals" / "overview" ): {
                GET: {
                    tags: { proposals },
                    summary: "Proposals overview",
                    description: "Get proposals overview.",
                    200: ProposalsOverview,
                }
            },
            ("proposals" / "search" ): {
                POST: {
                    tags: { proposals },
                    summary: "Proposals search",
                    description: "Get proposals data.",
                    body: requests::ProposalsRequest,
                    200: responses::ProposalsResponse,
                }
            },
            ("votes" / "search" ): {
                POST: {
                    tags: { votes },
                    summary: "Votes search",
                    description: "Get votes data.",
                    body: requests::VotesRequest,
                    200: responses::VotesResponse,
                }
            },
            ("voters" / { voter: String } / "search" ): {
                POST: {
                    tags: { voters },
                    summary: "Voter search",
                    description: "Get proposals with votes data.",
                    body: requests::VotersRequest,
                    200: responses::ProposalsWithVotesResponse,
                }
            },
            ("voters" / "proposals" / "count" ): {
                POST: {
                    tags: { voters },
                    summary: "Voter proposals count",
                    description: "Get proposals counts",
                    body: requests::ProposalsCountRequest,
                    200: Vec<responses::ProposalCountResponse>,
                }
            },
            ("voters" / "proposals" / "count" / "search" ): {
                POST: {
                    tags: { voters },
                    summary: "Voter proposals count search",
                    description: "Get proposals counts",
                    body: requests::ProposalsCountSearchRequest,
                    200: Vec<responses::ProposalCountResponse>,
                }
            },
        }
    };

    serde_yaml::to_string(&api).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_docs() {
        println!("{}", swagger());
    }
}
