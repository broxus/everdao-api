use std::str::FromStr;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    derive_more::Display,
    opg::OpgModel,
)]
#[opg("Timeframe")]
pub enum Timeframe {
    #[display(fmt = "H1")]
    H1,
    #[display(fmt = "D1")]
    D1,
}

impl FromStr for Timeframe {
    type Err = ();

    fn from_str(rename_all_str: &str) -> Result<Self, Self::Err> {
        match rename_all_str {
            "H1" => Ok(Timeframe::H1),
            "D1" => Ok(Timeframe::D1),
            _ => Err(()),
        }
    }
}
