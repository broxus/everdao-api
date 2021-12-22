use serde::Deserialize;

use crate::models::*;
use crate::utils::*;

pub type VotersSearch = Paginated<Ordered<VoterFilters, VotersOrdering>>;

pub type VotersProposalsCountSearch =
    Paginated<Ordered<VotersProposalsCountFilters, VotersProposalsOrdering>>;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct VoterFilters {
    pub start_time_ge: Option<u32>,
    pub start_time_le: Option<u32>,

    pub end_time_ge: Option<u32>,
    pub end_time_le: Option<u32>,

    pub proposal_id: Option<u32>,

    pub proposer: Option<String>,

    pub proposal_address: Option<String>,

    pub support: Option<bool>,

    pub locked: Option<bool>,

    pub available_for_unlock: Option<bool>,

    pub state: Option<ProposalState>,
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[opg("Voters ordering")]
pub struct VotersOrdering {
    pub column: VoterColumn,
    pub direction: Direction,
}

impl Default for VotersOrdering {
    fn default() -> Self {
        Self {
            column: VoterColumn::CreatedAt,
            direction: Direction::Descending,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Voter column")]
pub enum VoterColumn {
    CreatedAt,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct VotersProposalsCountFilters {
    pub voters: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[opg("Voters proposals ordering")]
pub struct VotersProposalsOrdering {
    pub column: VotersProposalColumn,
    pub direction: Direction,
}

impl Default for VotersProposalsOrdering {
    fn default() -> Self {
        Self {
            column: VotersProposalColumn::Count,
            direction: Direction::Descending,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Eq, PartialEq, Hash, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("Voter column")]
pub enum VotersProposalColumn {
    Count,
}
