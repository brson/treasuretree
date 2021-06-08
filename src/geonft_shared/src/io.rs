use anyhow::Result;
use geonft_request::{ClaimRequest, PlantRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, DirEntry, File, Metadata};
use std::io::{BufReader, BufWriter};

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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
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

fn get_sync_status(key: &str) -> Result<SyncStatus> {
    let path = format!("{}/{}", SYNC_STATUS_DIR, key);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let status: SyncStatus = serde_json::from_reader(reader)?;
    Ok(status)
}

pub fn record_sync_status(key: &str, status: SyncStatus) -> Result<()> {
    fs::create_dir_all(SYNC_STATUS_DIR)?;

    let path = format!("{}/{}", SYNC_STATUS_DIR, key);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &status)?;

    Ok(())
}

#[derive(Serialize)]
pub struct TreasureTemplateData {
    pub public_key: String,
    pub public_key_abbrev: String,
    pub public_url: String,
    pub image_url: String,
    pub planted_date_time: String,
    pub planted_by: String,
    pub claimed_date_time: String,
    pub claimed_by: String,
    pub sync_status: String,
}

pub fn load_treasure_data(public_key: &str) -> Result<TreasureTemplateData> {
    let public_key = public_key.to_string();

    let public_key_abbrev = geonft_nostd::abbrev_pubkey(&public_key);
    let public_url = format!("treasure/{}", public_key);
    let image_url = format!("treasure-images/{}", public_key);

    let plant_path = format!("{}/{}", PLANT_DIR, public_key);
    let plant_file = File::open(plant_path)?;
    let plant_meta = plant_file.metadata()?;
    let plant_time = plant_meta.modified()?;
    let plant_date_time = chrono::DateTime::<chrono::Local>::from(plant_time);
    let planted_date_time = plant_date_time.to_rfc2822();
    let plant_reader = BufReader::new(plant_file);
    let plant_request: PlantRequest = serde_json::from_reader(plant_reader)?;
    let planted_by = plant_request.account_public_key;

    let claim_path = format!("{}/{}", CLAIM_DIR, public_key);
    let claimed_date_time;
    let claimed_by;
    if let Ok(claim_file) = File::open(claim_path) {
        let claim_meta = claim_file.metadata()?;
        let claim_time = claim_meta.modified()?;
        let claim_date_time = chrono::DateTime::<chrono::Local>::from(claim_time);
        claimed_date_time = claim_date_time.to_rfc2822();
        let claim_reader = BufReader::new(claim_file);
        let claim_request: ClaimRequest = serde_json::from_reader(claim_reader)?;
        claimed_by = claim_request.account_public_key;
    } else {
        claimed_date_time = "unclaimed".to_string();
        claimed_by = "unclaimed".to_string();
    }

    let sync_status = get_ui_sync_status(&public_key)?;

    Ok(TreasureTemplateData {
        public_key,
        public_key_abbrev,
        public_url,
        image_url,
        planted_date_time,
        planted_by,
        claimed_date_time,
        claimed_by,
        sync_status,
    })
}

fn get_ui_sync_status(public_key: &str) -> Result<String> {
    let plant_path = format!("{}/{}", PLANT_DIR, public_key);
    let claim_path = format!("{}/{}", CLAIM_DIR, public_key);
    let have_plant = fs::metadata(plant_path).is_ok();
    let have_claim = fs::metadata(claim_path).is_ok();
    let sync_status = get_sync_status(public_key).ok();

    Ok(match (have_plant, have_claim, sync_status) {
        (false, _, _) => unreachable!(),
        (true, false, None | Some(SyncStatus::BlobSynced)) => "unsynced",
        (true, false, Some(SyncStatus::PlantSynced)) => "synced",
        (true, false, Some(SyncStatus::ClaimSynced)) => unreachable!(),
        (true, true, None | Some(SyncStatus::BlobSynced) | Some(SyncStatus::PlantSynced)) => {
            "unsynced"
        }
        (true, true, Some(SyncStatus::ClaimSynced)) => "synced",
    }
    .to_string())
}
