use serde::Deserialize;

use crate::models::*;
use crate::utils::*;

pub type VotersSearch = Paginated<Ordered<VoterFilters, VotersOrdering>>;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct VoterFilters {
    pub start_time_ge: Option<u32>,
    pub start_time_le: Option<u32>,

    pub end_time_ge: Option<u32>,
    pub end_time_le: Option<u32>,

    pub proposal_id: Option<u32>,

    pub proposer: Option<String>,

    pub support: Option<bool>,

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
