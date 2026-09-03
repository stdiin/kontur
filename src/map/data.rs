use rkyv::{Serialize, Deserialize, Archive};

use crate::map::object::MapObject;

#[derive(Debug, Serialize, Deserialize, Archive, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl From<Point> for egui_macroquad::macroquad::math::Vec2 {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Archive, Default, Clone)]
pub struct Category {
    pub name: String,
    pub objects: Vec<MapObject>,
}

#[derive(Debug, Serialize, Deserialize, Archive, Default, Clone)]
pub struct MapData {
    pub name: String,
    pub image: Vec<u8>,
    pub categories: Vec<Category>,
}
