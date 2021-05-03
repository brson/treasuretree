use anyhow::{Result, bail};
use rocket_contrib::{templates::Template, json::Json};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fs::{self, File, DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::fmt;
use treasure_qrcode::create_qr_code;
use crate::crypto;
use crate::treasure_qrcode;

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    secret_key: String,
    qrcode: String,
    url: String,
}

#[get("/api/create")]
pub fn create_treasure_key() -> Result<Json<CreateResponse>> {
    let init_keys = create_qr_code()?;
    let first_key = &init_keys[0];

    let first_key = CreateResponse {
        secret_key: first_key.secret_key.clone(),
        // Argument is the size, bigger number means smaller size on the page
        qrcode: first_key.qrcode.to_svg_string(0), 
        url: first_key.url.clone(),
    };

    Ok(Json(first_key))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantRequest {
    /// An image, base64 encoded
    pub image: String,
    /// A public key to represent the treasure, bech32 encoded
    pub public_key: String,
    /// A signature against the base64 encoded image by the corresponding private key
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantResponse {
    return_url: String,
}

/// Stores a treasure and associated key
///
/// Stores the json to disk,
/// with the encoded pubkey as the name of the file.
/// The key can be used later to retrieve (or claim) the treasure.
///
/// # Errors
///
/// If the signature is not a valid signature of the provided image with
/// the provided public key.
#[post("/api/plant", format = "json", data = "<plant_info>")]
pub fn plant_treasure_with_key(plant_info: Json<PlantRequest>) -> Result<Json<PlantResponse>> {
    let treasure_key = &plant_info.public_key;
    let filename = format!("data/treasure/{key}", key = treasure_key);
    let return_url = format!("{host}/api/plant/{key}\n", host = "http://localhost:8000", key = treasure_key);

    fs::create_dir_all("data/treasure")?;

    let mut file = File::create(filename)?;
    serde_json::to_writer(file, &plant_info.0)?;
    
    let res = PlantResponse {
        return_url,
    };
    
    Ok(Json(res))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimRequest {
    /// A random string signed by the private key as evidence of ownership
    nonce: String,
    /// The public key of the treasure, bech32 encoded
    public_key: String,
    /// A signature against the nonce by the corresponding private key
    signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimResponse {
    message: String,
    return_url: String,

}

/// Claim a treasure.
///
/// Checks that a treasure exists,
/// then verifies the signature with the public key and the nonce.
///
/// If the checks pass then store a record indicating
/// the treasure was claimed by the logged in user.
///
/// TODO: Add a user concept
#[post("/api/claim", format = "json", data = "<claim_info>")]
pub fn claim_treasure_with_key(claim_info: Json<ClaimRequest>) -> Result<Json<ClaimResponse>> {

    let res = (|| {
        // verify if it's a valid Public key
        let public_key_decode = crypto::decode_public_key(&claim_info.public_key)?;
        let public_key_encode = crypto::encode_public_key(&public_key_decode)?;

        dbg!(&claim_info);
        
        let filename = format!("data/treasure/{}", public_key_encode);
        if !Path::new(&filename).is_file() {
            bail!("Treasure doesn't exist")
        } else {
            let message = claim_info.nonce.as_bytes();
            let signature = crypto::decode_signature(&claim_info.signature)?;

            crypto::verify_signature(message, &signature, &public_key_decode)?;
            
            
            // todo:
            // - claim success and transfer asset 
            // - disable secret_key
            // - sync to blockchain

            let filename = format!("data/claim/{key}", key = public_key_encode);
            fs::create_dir_all("data/claim")?;

            println!("hihihi filename: {}", &filename);
            println!("hihihi claim info: {:?}", &claim_info.0);
            
            let mut file = File::create(filename)?;
            serde_json::to_writer(file, &claim_info.0)?;

            let return_url = format!("{host}/api/plant/{key}\n", host = "http://localhost:8000", key = public_key_encode);

            Ok(Json(ClaimResponse {
                message: format!("Congrats! Treasure received!"),
                return_url,
            }))
        }
    })();
    dbg!(res)
    
}

