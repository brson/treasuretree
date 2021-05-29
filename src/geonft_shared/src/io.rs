use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, DirEntry, File, Metadata};
use std::io::{BufReader, BufWriter};
use geonft_data::{PlantRequest, ClaimRequest};

pub static PLANT_DIR: &'static str = "data/plant";
pub static CLAIM_DIR: &'static str = "data/claim";
pub static SYNC_STATUS_DIR: &'static str = "data/sync-status";

/// A treasure key and the time its record was created,
/// can be used for both plants and claims.
pub struct TreasureTime {
    pub public_key: String,
    pub time: chrono::DateTime<chrono::Local>,
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
            dent.and_then(
                |(meta, dent): (Metadata, DirEntry)| -> Result<TreasureTime, _> {
                    let public_key = dent.file_name().into_string().expect("utf-8");
                    let time = meta.modified()?;
                    let time = chrono::DateTime::<chrono::Local>::from(time);
                    Ok(TreasureTime { public_key, time })
                },
            )
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

    let plants = plants.into_iter().map(|t| (PlantClaim::Plant, t));
    let claims = claims.into_iter().map(|t| (PlantClaim::Claim, t));

    let mut treasure_events: Vec<_> = plants.chain(claims).collect();

    treasure_events.sort_by_key(|(_, t)| t.time);

    Ok(treasure_events)
}

pub fn get_plant(key: &str) -> Result<PlantRequest> {
    fs::create_dir_all(PLANT_DIR)?;

    let path = format!("{}/{}", PLANT_DIR, key);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let req = serde_json::from_reader(reader)?;

    Ok(req)
}

pub fn get_claim(key: &str) -> Result<ClaimRequest> {
    fs::create_dir_all(CLAIM_DIR)?;

    let path = format!("{}/{}", CLAIM_DIR, key);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let req = serde_json::from_reader(reader)?;

    Ok(req)
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub enum SyncStatus {
    BlobSynced,
    PlantSynced,
    ClaimSynced,
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

pub fn record_sync_status(key: &str, status: SyncStatus) -> Result<()> {
    fs::create_dir_all(SYNC_STATUS_DIR)?;

    let path = format!("{}/{}", SYNC_STATUS_DIR, key);

    let file = File::open(path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &status)?;

    Ok(())
}
