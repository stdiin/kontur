use rkyv::{Serialize, Deserialize, Archive};

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub enum MapObject {
    Peak(Peak),
    Region(Region),
    River(River)
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

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct Peak {
    pub name: String
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct Region {
    pub name: String
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct River {
    pub name: String
}