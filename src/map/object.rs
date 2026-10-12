use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub enum MapObjectKind {
    Peak,
    Region,
    River,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct MapObject {
    pub name: String,
    pub kind: MapObjectKind,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct Peak {
    pub name: String,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct Region {
    pub name: String,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct River {
    pub name: String,
}
