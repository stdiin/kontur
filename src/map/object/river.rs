use rkyv::{Archive, Serialize, Deserialize};

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
pub struct River {
    pub name: String
}
