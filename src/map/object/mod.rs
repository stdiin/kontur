use rkyv::{Archive, Deserialize, Serialize};

mod peak;
mod river;
mod region;

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub enum MapObject {
    Peak(peak::Peak),
    Region(region::Region),
    River(river::River)
}

impl MapObject {
    pub fn name(&self) -> &str {
        match self {
            Self::Peak(peak) => &peak.name,
            Self::Region(region) => &region.name,
            Self::River(river) => &river.name
        }
    }
}
