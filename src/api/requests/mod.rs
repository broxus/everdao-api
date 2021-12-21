use serde::Deserialize;

use crate::models::*;
use crate::utils::*;

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
    pub proposal_address: Option<String>,

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
            proposal_address: w.proposal_address,
            state: w.state,
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
    pub locked: Option<bool>,

    #[opg(optional)]
    pub ordering: Option<VotesOrdering>,
}

impl From<VotesRequest> for VotesSearch {
    fn from(w: VotesRequest) -> Self {
        VoteFilters {
            proposal_id: w.proposal_id,
            voter: w.voter,
            support: w.support,
            locked: w.locked,
        }
        .ordered(w.ordering)
        .paginated(w.limit, w.offset)
    }
}

#[derive(Debug, Deserialize, Clone, opg::OpgModel)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[opg("Voters request")]
pub struct VotersRequest {
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
    pub proposal_address: Option<String>,

    #[opg(optional)]
    pub support: Option<bool>,

    #[opg(optional)]
    pub locked: Option<bool>,

    #[opg(optional)]
    pub state: Option<ProposalState>,

    #[opg(optional)]
    pub ordering: Option<VotersOrdering>,
}

impl From<VotersRequest> for VotersSearch {
    fn from(w: VotersRequest) -> Self {
        VoterFilters {
            start_time_ge: w.start_time_ge,
            start_time_le: w.start_time_le,
            end_time_ge: w.end_time_ge,
            end_time_le: w.end_time_le,
            proposal_id: w.proposal_id,
            proposer: w.proposer,
            proposal_address: w.proposal_address,
            support: w.support,
            locked: w.locked,
            state: w.state,
        }
        .ordered(w.ordering)
        .paginated(w.limit, w.offset)
    }
}

#[derive(Debug, Clone, Deserialize, opg::OpgModel)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[opg("Proposals count request")]
pub struct ProposalsCountRequest {
    pub voters: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, opg::OpgModel)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[opg("Proposals count search request")]
pub struct ProposalsCountSearchRequest {
    pub limit: u32,
    pub offset: u32,

    #[opg(optional)]
    pub voters: Option<Vec<String>>,

    #[opg(optional)]
    pub ordering: Option<VotersProposalsOrdering>,
}

impl From<ProposalsCountSearchRequest> for VotersProposalsCountSearch {
    fn from(w: ProposalsCountSearchRequest) -> Self {
        VotersProposalsCountFilters { voters: w.voters }
            .ordered(w.ordering)
            .paginated(w.limit, w.offset)
    }
}
