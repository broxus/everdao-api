#![allow(clippy::needless_update)]

use opg::*;

use crate::api::requests;
use crate::api::responses;

pub fn swagger() -> String {
    let api = describe_api! {
        info: {
            title: "DAO",
            version: "1.0.0",
            description: r##"This API allows you to get the information of DAO"##,
        },
        servers: {
            "https://bridge-dao-indexer-test.broxus.com/v1",
            "https://bridge-dao-indexer.broxus.com/v1"
        },
        tags: {
            proposals,
            voters,
            votes,
        },
        paths: {
            ("proposals"): {
                POST: {
                    tags: { proposals },
                    summary: "Proposals data",
                    description: "Get proposals data by id's.",
                    body: requests::ProposalsByIdRequest,
                    200: Vec<responses::ProposalResponse>,
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
