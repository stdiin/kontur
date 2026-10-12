use crate::map::object::MapObject;
use rkyv::{Archive, Deserialize, Serialize};
use std::{fs, io, path::Path};

pub mod object;
pub mod viewer;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MapLoadError {
    IOError(io::Error),
    DecompressionError(io::Error),
    DeserializationError(rkyv::rancor::Error),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MapSaveError {
    IOError(io::Error),
    CompressionError(io::Error),
    SerializationError(rkyv::rancor::Error),
}

#[allow(dead_code)]
pub fn load(path: impl AsRef<Path>) -> Result<MapData, MapLoadError> {
    let path = path.as_ref();
    let bytes_compressed = fs::read(path).map_err(MapLoadError::IOError)?;

    let bytes =
        zstd::decode_all::<&[u8]>(&bytes_compressed).map_err(MapLoadError::DecompressionError)?;

    rkyv::from_bytes::<MapData, rkyv::rancor::Error>(&bytes)
        .map_err(MapLoadError::DeserializationError)
}

#[allow(dead_code)]
pub fn save(map_data: impl Into<MapData>, path: impl AsRef<Path>) -> Result<(), MapSaveError> {
    let path = path.as_ref();
    let data = map_data.into();
    let bytes =
        rkyv::to_bytes::<rkyv::rancor::Error>(&data).map_err(MapSaveError::SerializationError)?;

    let compressed = zstd::encode_all(bytes.as_ref(), 0).map_err(MapSaveError::CompressionError)?;

    fs::write(path, compressed).map_err(MapSaveError::IOError)
}

#[derive(Debug, Serialize, Deserialize, Archive, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
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
