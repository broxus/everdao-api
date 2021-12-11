use serde::Deserialize;

use crate::models::*;
use crate::utils::*;

#[derive(Debug, Clone, Deserialize, opg::OpgModel)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[opg("Proposal by id request")]
pub struct ProposalByIdRequest {
    pub id: u32,
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[opg("Proposals request")]
pub struct ProposalsRequest {
    pub limit: u32,
    pub offset: u32,

    #[opg(optional)]
    pub start_time_ge: Option<u32>,
    #[opg(optional)]
    pub start_time_le: Option<u32>,

    #[opg(optional)]
    pub end_time_ge: Option<u32>,
    #[opg(optional)]
    pub end_time_le: Option<u32>,

    #[opg(optional)]
    pub proposal_id: Option<u32>,

    #[opg(optional)]
    pub proposer: Option<String>,

    #[opg(optional)]
    pub state: Option<ProposalState>,

    #[opg(optional)]
    pub ordering: Option<ProposalsOrdering>,
}

impl From<ProposalsRequest> for ProposalsSearch {
    fn from(w: ProposalsRequest) -> Self {
        ProposalFilters {
            start_time_ge: w.start_time_ge,
            start_time_le: w.start_time_le,
            end_time_ge: w.end_time_ge,
            end_time_le: w.end_time_le,
            proposal_id: w.proposal_id,
            proposer: w.proposer,
            state: w.state,
            voter: None,
        }
        .ordered(w.ordering)
        .paginated(w.limit, w.offset)
    }
}

#[derive(Debug, serde::Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Proposal votes request")]
pub struct VotesRequest {
    pub limit: u32,
    pub offset: u32,

    #[opg(optional)]
    pub proposal_id: Option<u32>,

    #[opg(optional)]
    pub voter: Option<String>,

    #[opg(optional)]
    pub support: Option<bool>,

    #[opg(optional)]
    pub ordering: Option<VotesOrdering>,
}

impl From<VotesRequest> for VotesSearch {
    fn from(w: VotesRequest) -> Self {
        VoteFilters {
            proposal_id: w.proposal_id,
            voter: w.voter,
            support: w.support,
        }
        .ordered(w.ordering)
        .paginated(w.limit, w.offset)
    }
}
