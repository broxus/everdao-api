#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Stakeholders ordering")]
pub enum StakeholdersOrdering {
    UpdateAtAscending,
    UpdateAtDescending,
    StakeAscending,
    StakeDescending,
    FrozenStakeAscending,
    FrozenStakeDescending,
    LastRewardAscending,
    LastRewardDescending,
    TotalRewardAscending,
    TotalRewardDescending,
    CreatedAtAscending,
    CreatedAtDescending,
}
