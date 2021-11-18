#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[serde(rename_all = "lowercase")]
#[opg("Stakeholders ordering")]
pub enum TransfersOrdering {
    VolumeExecAscending,
    VolumeExecDescending,
    UpdateAtAscending,
    UpdateAtDescending,
    CreatedAtAscending,
    CreatedAtDescending,
}
