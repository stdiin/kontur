use std::{error::Error, fs, path::Path};

pub mod data;
pub mod object;
pub mod viewer;

use data::MapData;

pub fn load(path: impl AsRef<Path>) -> Option<MapData> {
    let path = path.as_ref();
    let bytes = fs::read(path).ok()?;
    rkyv::from_bytes::<MapData, rkyv::rancor::Error>(&bytes).ok()
}

pub fn save(map_data: impl Into<MapData>, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let data = map_data.into();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&data)?;

    fs::write(path, bytes)?;

    Ok(())
}