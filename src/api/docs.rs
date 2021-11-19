#![allow(clippy::needless_update)]

use opg::*;

use crate::api::{requests, responses};

pub fn swagger() -> String {
    let api = describe_api! {
        info: {
            title: "Farming",
            version: "1.0.0",
            description: r##"This API allows you to get the information on farm for farming pool"##,
        },
        servers: {
            "https://farming-pool-indexer-test.broxus.com/v1",
            "https://farming-pool-indexer.broxus.com/v1"
        },
        tags: {
            staking,
            transfers
        },
        paths: {
            ("staking" / "search" / "stakeholders"): {
                POST: {
                    tags: { staking },
                    summary: "Stakeholders search",
                    description: "Get stakeholders data.",
                    body: requests::SearchProposalsRequest,
                    200: responses::ProposalsResponse,
                }
            },
            ("staking" / "search" / "transactions"): {
                POST: {
                    tags: { staking },
                    summary: "Transactions search",
                    description: "Get transactions data.",
                    body: requests::SearchTransactionsRequest,
                    200: responses::TransactionsTableResponse,
                }
            },
            ("staking" / "graph" / "tvl"): {
                POST: {
                    tags: { staking },
                    summary: "Graph tvl",
                    description: "Get tvl graph data.",
                    body: requests::GraphRequest,
                    200: responses::GraphDataResponse,
                }
            },
            ("staking" / "graph" / "apr"): {
                POST: {
                    tags: { staking },
                    summary: "Graph apr",
                    description: "Get apr graph data.",
                    body: requests::GraphRequest,
                    200: responses::GraphDataResponse,
                }
            },
            ("staking"): {
                GET: {
                    tags: { staking },
                    summary: "Main page staking",
                    description: "Get main page data.",
                    200: responses::MainPageStakingResponse,
                }
            },
            ("staking"): {
                POST: {
                    tags: { staking },
                    summary: "User page staking",
                    description: "Post user page data.",
                    body: requests::UserPageStakingRequest,
                    200: responses::UserPageStakingResponse,
                }
            },
            ("transfers" / "search"): {
                POST: {
                    tags: { transfers },
                    summary: "Transfers search",
                    description: "Get transfers data.",
                    body: requests::SearchProposalRequest,
                    200: responses::VotesResponse,
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
