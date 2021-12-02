use std::str::FromStr;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Eq, PartialEq, Hash, opg::OpgModel,
)]
#[opg("Vote Ordering")]
pub enum VoteOrdering {
    CreatedAtDesc,
    CreatedAtAsc,
}

impl ToString for VoteOrdering {
    fn to_string(&self) -> String {
        match self {
            VoteOrdering::CreatedAtDesc => "CreatedAtDesc".into(),
            VoteOrdering::CreatedAtAsc => "CreatedAtAsc".into(),
        }
    }
}

impl FromStr for VoteOrdering {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "createdatdesc" => Ok(Self::CreatedAtDesc),
            "createdatasc" => Ok(Self::CreatedAtAsc),
            &_ => Err(anyhow::Error::msg(format!("invalid event type {}", s))),
        }
    }
}
