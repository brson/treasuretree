//! Rocket API calls
//!
//! These are all mounted under `/api/` and only used from JS.

use crate::crypto;
use anyhow::{bail, Result};
use geonft_request::{ClaimRequest, PlantRequest};
use geonft_shared::io;
use rocket_contrib::{json::Json, templates::Template};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantResponse;

/// Stores a treasure and associated key
///
/// Validation:
///
/// - The treasure has not already been planted.
/// - The image can be decoded and has an acceptable type.
/// - The account key is valid.
/// - The treasure key is valid.
/// - The account signature.
/// - The treasure signature.
/// - The account public key is an authorized treasure planter.
///
/// Stores the json to disk,
/// with the encoded pubkey as the name of the file.
/// The pubkey can be used later to retrieve (or claim) the treasure.
#[post("/api/plant", format = "json", data = "<plant_info>")]
pub fn plant_treasure_with_key(plant_info: Json<PlantRequest>) -> Result<Json<PlantResponse>> {
    let treasure_key_decode = crypto::decode_treasure_public_key(&plant_info.treasure_public_key)?;
    let treasure_key_encode = crypto::encode_treasure_public_key(&treasure_key_decode)?;

    let account_key_decode = crypto::decode_account_public_key(&plant_info.account_public_key)?;

    let treasure_signature = crypto::decode_signature(&plant_info.treasure_signature)?;
    let account_signature = crypto::decode_signature(&plant_info.account_signature)?;

    let filename = format!("{}/{}", io::PLANT_DIR, treasure_key_encode);
    if Path::new(&filename).is_file() {
        bail!("Treasure already exists")
    }

    let treasure_hash = crypto::get_hash(&plant_info.image)?;

    crypto::verify_plant_request_for_treasure(
        &treasure_key_decode,
        &account_key_decode,
        treasure_hash.as_bytes(),
        &treasure_signature,
    )?;

    crypto::verify_plant_request_for_account(
        &account_key_decode,
        &treasure_key_decode,
        &account_signature,
    )?;

    let filename = format!("{}/{key}", io::PLANT_DIR, key = treasure_key_encode);
    fs::create_dir_all(io::PLANT_DIR)?;

    let mut file = File::create(filename)?;
    serde_json::to_writer(file, &plant_info.0)?;

    Ok(Json(PlantResponse))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimResponse;

/// Claim a treasure.
///
/// Validation:
///
/// - The account key is valid.
/// - The treasure key is valid.
/// - The treasure exists,
/// - The account signature.
/// - The treasure signature.
///
/// If the checks pass then store a record indicating
/// the treasure was claimed by the logged in user.
#[post("/api/claim", format = "json", data = "<claim_info>")]
pub fn claim_treasure_with_key(claim_info: Json<ClaimRequest>) -> Result<Json<ClaimResponse>> {
    let treasure_key_decode = crypto::decode_treasure_public_key(&claim_info.treasure_public_key)?;
    let treasure_key_encode = crypto::encode_treasure_public_key(&treasure_key_decode)?;

    let filename = format!("{}/{}", io::PLANT_DIR, treasure_key_encode);
    if !Path::new(&filename).is_file() {
        bail!("Treasure doesn't exist")
    }

    let account_key_decode = crypto::decode_account_public_key(&claim_info.account_public_key)?;
    let treasure_signature = crypto::decode_signature(&claim_info.treasure_signature)?;
    let account_signature = crypto::decode_signature(&claim_info.account_signature)?;

    crypto::verify_claim_request_for_treasure(
        &treasure_key_decode,
        &account_key_decode,
        &treasure_signature,
    )?;

    crypto::verify_claim_request_for_account(
        &account_key_decode,
        &treasure_key_decode,
        &account_signature,
    )?;

    let filename = format!("{}/{key}", io::CLAIM_DIR, key = treasure_key_encode);
    fs::create_dir_all(io::CLAIM_DIR)?;

    let mut file = File::create(filename)?;
    serde_json::to_writer(file, &claim_info.0)?;

    let return_url = format!(
        "{host}/treasure/{key}\n",
        host = "http://localhost:8000",
        key = treasure_key_encode
    );

    Ok(Json(ClaimResponse))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreasureRequest {
    treasure_public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreasureResponse {
    treasure_exists: bool,
}

#[post("/api/exists", format = "json", data = "<treasure>")]
pub fn treasure_exists(treasure: Json<TreasureRequest>) -> Result<Json<TreasureResponse>> {
    let treasure_key_decode = crypto::decode_treasure_public_key(&treasure.treasure_public_key)?;
    let treasure_key_encode = crypto::encode_treasure_public_key(&treasure_key_decode)?;

    let filename = format!("{}/{}", io::PLANT_DIR, treasure_key_encode);
    println!("filename: {}", filename);

    if Path::new(&filename).is_file() {
        Ok(Json(TreasureResponse {
            treasure_exists: true,
        }))
    } else {
        Ok(Json(TreasureResponse {
            treasure_exists: false,
        }))
    }
}
