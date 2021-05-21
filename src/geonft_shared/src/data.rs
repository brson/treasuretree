use anyhow::Result;
use std::fs::{self, DirEntry, Metadata, File};
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub static PLANT_DIR: &'static str = "data/plant";
pub static CLAIM_DIR: &'static str = "data/claim";
pub static SYNC_STATUS_DIR: &'static str = "data/sync-status";

/// A treasure key and the time its record was created,
/// can be used for both plants and claims.
pub struct TreasureTime {
    pub public_key: String,
    pub time: chrono::DateTime::<chrono::Local>,
}

pub fn get_all_planted_treasures() -> Result<Vec<TreasureTime>> {
    get_all_treasures_from_dir(PLANT_DIR)
}

pub fn get_all_claimed_treasures() -> Result<Vec<TreasureTime>> {
    get_all_treasures_from_dir(CLAIM_DIR)
}

fn get_all_treasures_from_dir(dir: &str) -> Result<Vec<TreasureTime>> {
    fs::create_dir_all(dir)?;
    let files = fs::read_dir(dir)?
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
            dent.and_then(|(meta, dent): (Metadata, DirEntry)| -> Result<TreasureTime, _> {
                let public_key = dent.file_name().into_string().expect("utf-8");
                let time = meta.modified()?;
                let time = chrono::DateTime::<chrono::Local>::from(time);
                Ok(TreasureTime {
                    public_key, time,
                })
            })
        })
        // Collect iter of Result into Result<Vec>,
        // and return any error.
        .collect::<Result<Vec<_>, _>>()?;

    Ok(files)
}

pub fn get_all_planted_treasures_time_sorted() -> Result<Vec<TreasureTime>> {
    let mut treasures = get_all_planted_treasures()?;

    treasures.sort_by_key(|t| t.time);

    Ok(treasures)
}

pub fn get_all_claimed_treasures_time_sorted() -> Result<Vec<TreasureTime>> {
    let mut treasures = get_all_claimed_treasures()?;

    treasures.sort_by_key(|t| t.time);

    Ok(treasures)
}

pub enum PlantClaim {
    Plant,
    Claim,
}

pub fn get_all_plants_and_claims_time_sorted() -> Result<Vec<(PlantClaim, TreasureTime)>> {
    let plants = get_all_planted_treasures_time_sorted()?;
    let claims = get_all_claimed_treasures_time_sorted()?;

    let plants: Vec<_> = plants.into_iter().map(|t| (PlantClaim::Plant, t)).collect();
    let claims: Vec<_> = claims.into_iter().map(|t| (PlantClaim::Claim, t)).collect();

    let mut treasure_events: Vec<_> = plants.into_iter().chain(claims.into_iter()).collect();

    treasure_events.sort_by_key(|(_, t)| t.time);

    Ok(treasure_events)
}

#[derive(Serialize, Deserialize)]
pub enum SyncStatus {
    Planted,
    PlantedAndBlobSynced,
    PlantedAndSynced,
    Claimed,
    ClaimedAndBlobSynced,
    ClaimedAndPlantSynced,
    ClaimedAndFullySynced,
}

pub fn get_all_sync_statuses() -> Result<HashMap<String, SyncStatus>> {
    fs::create_dir_all(SYNC_STATUS_DIR)?;

    let mut statuses = HashMap::new();

    for dent in fs::read_dir(SYNC_STATUS_DIR)? {
        let dent = dent?;

        let meta = dent.metadata()?;

        if !meta.is_file() {
            continue;
        }

        let public_key = dent.file_name().into_string().expect("utf-8");

        let file = File::open(dent.path())?;
        let mut reader = BufReader::new(file);

        let status: SyncStatus = serde_json::from_reader(&mut reader)?;

        statuses.insert(public_key, status);
    }

    Ok(statuses)
}
