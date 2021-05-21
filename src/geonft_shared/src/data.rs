use anyhow::Result;
use std::fs::{self, DirEntry, Metadata};

pub static PLANT_DIR: &'static str = "data/plant";
pub static CLAIM_DIR: &'static str = "data/claim";

pub struct PlantedTreasure {
    pub public_key: String,
    pub time: chrono::DateTime::<chrono::Local>,
}

pub fn get_all_planted_treasures() -> Result<Vec<PlantedTreasure>> {
    let files = fs::read_dir(PLANT_DIR)?
        // Get the file metadata
        .map(|dent: Result<DirEntry, _>| dent.and_then(|dent| Ok((dent.metadata()?, dent))))
        // Only keep entries that are files or errors
        .filter(|dent: &Result<(Metadata, DirEntry), _>| {
            dent.as_ref()
                .map(|(meta, _)| meta.is_file())
                .unwrap_or(true)
        })
        // Keep modify time for sorting
        .map(|dent: Result<(Metadata, DirEntry), _>| {
            dent.and_then(|(meta, dent): (Metadata, DirEntry)| -> Result<PlantedTreasure, _> {
                let public_key = dent.file_name().into_string().expect("utf-8");
                let time = meta.modified()?;
                let time = chrono::DateTime::<chrono::Local>::from(time);
                Ok(PlantedTreasure {
                    public_key, time,
                })
            })
        })
        // Collect iter of Result into Result<Vec>,
        // and return any error.
        .collect::<Result<Vec<_>, _>>()?;

    Ok(files)
}
