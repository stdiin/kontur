use rkyv::{Serialize, Deserialize, Archive};

#[derive(Debug, Serialize, Deserialize, Archive, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Archive, Default, Clone)]
pub struct Region {
    pub name: String,
    pub vertices: Vec<Point>,
}

#[derive(Debug, Serialize, Deserialize, Archive, Default, Clone)]
pub struct Category {
    pub name: String,
    pub regions: Vec<Region>,
}

#[derive(Debug, Serialize, Deserialize, Archive, Default, Clone)]
pub struct MapData {
    pub name: String,
    pub image: Vec<u8>,
    pub categories: Vec<Category>,
}
