use std::{io, fs, path::Path};
use rkyv::{Serialize, Deserialize, Archive};

pub mod peak;
pub mod river;
pub mod region;
pub mod viewer;

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

pub enum MapLoadError {
    IOError(io::Error),
    DecompressionError(io::Error),
    DeserializationError(rkyv::rancor::Error),
}

pub enum MapSaveError {
    IOError(io::Error),
    CompressionError(io::Error),
    SerializationError(rkyv::rancor::Error),
}

pub fn load(path: impl AsRef<Path>) -> Result<MapData, MapLoadError> {
    let path = path.as_ref();
    let bytes_compressed = fs::read(path).map_err(|e| MapLoadError::IOError(e))?;

    let bytes = zstd::decode_all::<&[u8]>(&bytes_compressed)
        .map_err(|e| MapLoadError::DecompressionError(e))?;

    rkyv::from_bytes::<MapData, rkyv::rancor::Error>(&bytes)
        .map_err(|e| MapLoadError::DeserializationError(e))
}

pub fn save(map_data: impl Into<MapData>, path: impl AsRef<Path>) -> Result<(), MapSaveError> {
    let path = path.as_ref();
    let data = map_data.into();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&data)
        .map_err(|e| MapSaveError::SerializationError(e))?;

    let compressed =
        zstd::encode_all(bytes.as_ref(), 0).map_err(|e| MapSaveError::CompressionError(e))?;

    fs::write(path, compressed).map_err(|e| MapSaveError::IOError(e))
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